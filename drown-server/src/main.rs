use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};

use drown_common::proto::{
    decode::DecodedPacket,
    encode::EncodedPacket,
    packet::{C2SPacket, S2CPacket, S2CQuerySuccessResponsePacket},
};
use futures::{SinkExt, TryStreamExt};
use once_cell::sync::Lazy;
use tokio::{
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

#[derive(Debug)]
struct ClientConnection {
    write_stream: FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>,
    keep_alive_state: KeepAliveState,
    last_message_id: u32,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl Drop for ClientConnection {
    fn drop(&mut self) {
        for task in self.tasks.drain(..) {
            task.abort();
        }
    }
}

impl ClientConnection {
    fn inc_message_id(&mut self) -> u32 {
        self.last_message_id += 1;
        self.last_message_id
    }

    async fn send_packet(&mut self, packet: EncodedPacket<S2CPacket>) {
        self.write_stream
            .send(packet.to_bytes())
            .await
            .expect("Failed to send keep-alive packet")
    }
}

#[derive(Debug)]
enum KeepAliveState {
    WaitingForRequest {
        response_received_at: std::time::Instant,
    },
    WaitingForResponse {
        request_message_id: u32,
        request_sent_at: std::time::Instant,
    },
}

impl Default for KeepAliveState {
    fn default() -> Self {
        Self::WaitingForRequest {
            response_received_at: std::time::Instant::now(),
        }
    }
}

static CLIENT_CONNECTIONS: Lazy<Arc<Mutex<HashMap<SocketAddr, ClientConnection>>>> =
    Lazy::new(Default::default);

async fn handle_socket_connection(socket: TcpStream) -> std::io::Result<()> {
    let addr = socket.peer_addr()?;
    let (read, write) = socket.into_split();

    // Create a length-delimited reader and writer
    let mut delimited_reader = FramedRead::new(read, LengthDelimitedCodec::new());
    let delimited_writer = FramedWrite::new(write, LengthDelimitedCodec::new());

    // Obtain a lock on the client connections map
    let mut client_connections = CLIENT_CONNECTIONS.lock().await;

    // Create a new client connection
    let mut client_connection = ClientConnection {
        write_stream: delimited_writer,
        keep_alive_state: KeepAliveState::default(),
        last_message_id: 0,
        tasks: Vec::new(),
    };

    // Spawn a task for periodically sending keep-alive packets
    client_connection.tasks.push(tokio::spawn(async move {
        const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(60);
        const KEEP_ALIVE_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

        loop {
            // Sleep first to allow dropping the mutex lock immediately after the loop iteration
            tokio::time::sleep(Duration::from_secs(1)).await;

            let mut client_connections = CLIENT_CONNECTIONS.lock().await;

            let client_connection = client_connections.get_mut(&addr).unwrap();

            match client_connection.keep_alive_state {
                KeepAliveState::WaitingForRequest {
                    response_received_at,
                } => {
                    // Do nothing if the last response was received less than 60 seconds ago
                    if response_received_at.elapsed() < KEEP_ALIVE_INTERVAL {
                        continue;
                    }

                    // Send a keep-alive request
                    let req = EncodedPacket::from_payload(S2CPacket::KeepAliveRequest)
                        .with_id(client_connection.inc_message_id());
                    client_connection.send_packet(req).await;

                    // Update the keep-alive state
                    client_connection.keep_alive_state = KeepAliveState::WaitingForResponse {
                        request_message_id: client_connection.last_message_id,
                        request_sent_at: std::time::Instant::now(),
                    };
                }
                KeepAliveState::WaitingForResponse {
                    request_sent_at, ..
                } => {
                    // If the last request was sent less than 30 seconds ago, wait for a response
                    if request_sent_at.elapsed() < KEEP_ALIVE_RESPONSE_TIMEOUT {
                        continue;
                    }

                    // If the last request was sent more than 30 seconds ago, disconnect the client
                    println!("Client {:?} timed out", addr);
                    client_connections.remove(&addr);
                }
            }
        }
    }));

    // Spawn a task for handling incoming packets
    client_connection.tasks.push(tokio::spawn(async move {
        while let Some(msg) = delimited_reader.try_next().await.unwrap() {
            let packet = DecodedPacket::<C2SPacket>::from_bytes(msg).unwrap();

            let mut client_connections = CLIENT_CONNECTIONS.lock().await;
            let client_connection = client_connections.get_mut(&addr).unwrap();

            match packet.payload() {
                C2SPacket::KeepAliveResponse => match client_connection.keep_alive_state {
                    KeepAliveState::WaitingForResponse {
                        request_message_id, ..
                    } => {
                        if packet.response_to() == Some(request_message_id) {
                            client_connection.keep_alive_state = KeepAliveState::WaitingForRequest {
                                response_received_at: std::time::Instant::now(),
                            };
                        } else {
                            eprintln!(
                                "Got KeepAliveResponse with unexpected response_to: {:?}",
                                packet
                            )
                        }
                    }
                    KeepAliveState::WaitingForRequest { .. } => {
                        eprintln!("Got unexpected KeepAliveResponse packet: {:?}", packet)
                    }
                },
                C2SPacket::QueryRequest(_query_request) => {
                    // TODO: Parse the SQL query and execute it

                    let res = EncodedPacket::from_payload(S2CPacket::QueryResponse(Ok(
                        S2CQuerySuccessResponsePacket {
                            schema: "this is some schema".to_string(),
                            data: vec!["this is a data row".to_string()],
                        },
                    )))
                    .with_id(client_connection.inc_message_id())
                    .with_response_to(packet.message_id());

                    client_connection.send_packet(res).await;
                }
            }
        }
    }));

    // Insert the client connection into the map
    client_connections.insert(addr, client_connection);

    Ok(())
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    // Bind a server socket
    let listener = TcpListener::bind("127.0.0.1:6472").await.unwrap();

    println!("listening on drown://{:?}", listener.local_addr().unwrap());

    loop {
        let (socket, _) = listener.accept().await?;
        handle_socket_connection(socket).await.unwrap();
    }
}
