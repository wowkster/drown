#![allow(dead_code)]

use std::marker::PhantomData;

use bytes::{Bytes, BytesMut};

use super::{header::PacketHeader, packet::PacketPayload};

pub struct EncodedPacket<P: PacketPayload> {
    header: PacketHeader,
    payload: Bytes,
    _payload_type: PhantomData<P>,
}

impl<P: PacketPayload> EncodedPacket<P> {
    pub fn from_payload(payload: P) -> Self {
        let payload = serde_json::to_vec(&payload).unwrap();
        let header = PacketHeader::default();

        Self {
            header,
            payload: payload.into(),
            _payload_type: PhantomData,
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.header.message_id = id;
        self
    }

    pub fn with_response_to(mut self, id: u32) -> Self {
        self.header.response_to = Some(id);
        self
    }

    pub fn message_id(&self) -> u32 {
        self.header.message_id
    }

    pub fn response_to(&self) -> Option<u32> {
        self.header.response_to
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(PacketHeader::SIZE + self.payload.len());

        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.payload);

        bytes.freeze()
    }
}
