use drown_common::proto::decode::DecodedPacket;
use drown_common::proto::encode::EncodedPacket;
use drown_common::proto::packet::C2SPacket;
use drown_common::proto::packet::S2CPacket;
use futures::SinkExt;
use futures::TryStreamExt;
use std::error::Error;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::cli::get_client_options_from_args;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = get_client_options_from_args();
    println!("{:#?}", args);

    // Connect to a peer
    let mut stream =
        TcpStream::connect(format!("{}:{}", args.connection.host, args.connection.port)).await?;

    let (read, write) = stream.split();

    let mut delimited_reader = FramedRead::new(read, LengthDelimitedCodec::new());
    let mut delimited_writer = FramedWrite::new(write, LengthDelimitedCodec::new());

    let message_id = AtomicU32::new(0);

    while let Some(msg) = delimited_reader.try_next().await.unwrap() {
        let packet = DecodedPacket::<S2CPacket>::from_bytes(msg).unwrap();

        match packet.payload() {
            S2CPacket::KeepAliveRequest => {
                let res = EncodedPacket::from_payload(C2SPacket::KeepAliveResponse)
                    .with_id(message_id.fetch_add(1, Ordering::SeqCst))
                    .with_response_to(packet.message_id());

                delimited_writer.send(res.to_bytes()).await.unwrap();
            }
            S2CPacket::QueryResponse(_) => todo!(),
        }
    }

    Ok(())
}

// struct StatementParser;

// #[derive(Debug, Error)]
// enum StatementParseError {
//     #[error("Invalid statement")]
//     InvalidStatement,
// }

// impl StatementParser {
//     fn parse(statement: &str) -> Result<(), StatementParseError> {
//         if statement.starts_with("SELECT") {
//             Ok(())
//         } else {
//             Err(StatementParseError::InvalidStatement)
//         }
//     }
// }
