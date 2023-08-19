use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait PacketPayload: Serialize + for<'a> Deserialize<'a> + Debug {}

impl PacketPayload for C2SPacket {}
impl PacketPayload for S2CPacket {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum C2SPacket {
    KeepAliveResponse,
    QueryRequest(C2SQueryRequestPacket),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct C2SQueryRequestPacket {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum S2CPacket {
    KeepAliveRequest,
    QueryResponse(Result<S2CQuerySuccessResponsePacket, S2CQueryErrorResponsePacket>),
}

// TODO: Create a data structure for this
#[derive(Debug, Serialize, Deserialize)]
pub struct S2CQuerySuccessResponsePacket {
    pub schema: String,
    pub data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S2CQueryErrorResponsePacket {
    pub error: String,
}
