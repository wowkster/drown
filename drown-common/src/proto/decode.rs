#![allow(dead_code)]

use std::ops::Deref;

use bytes::BytesMut;
use thiserror::Error;

use super::{header::PacketHeader, packet::PacketPayload};

/// Read-only representation of a packet sent from the client to the server.
///
/// This struct is used to represent a packet that has been received from the client.
#[derive(Debug)]
pub struct DecodedPacket<P: PacketPayload> {
    header: PacketHeader,
    payload: P,
}

/// An error that can occur when decoding a packet.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// The packet was not long enough to contain a full header.
    #[error("Packet length is invalid")]
    InvalidHeaderLength,

    /// The packet payload type did not match the expected type
    #[error("Failed to decode packet payload")]
    PayloadDecodeError,
}

impl<P: PacketPayload> DecodedPacket<P> {
    /// Decode a packet from a byte buffer.
    pub fn from_bytes(bytes: BytesMut) -> Result<Self, DecodeError> {
        if bytes.len() < PacketHeader::SIZE {
            return Err(DecodeError::InvalidHeaderLength);
        }

        let header = PacketHeader::from_bytes(&bytes[0..PacketHeader::SIZE]);

        let payload = serde_json::from_slice::<P>(&bytes[PacketHeader::SIZE..])
            .map_err(|_| DecodeError::PayloadDecodeError)?;

        Ok(Self { header, payload })
    }

    pub fn message_id(&self) -> u32 {
        self.header.message_id
    }

    pub fn response_to(&self) -> Option<u32> {
        self.header.response_to
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<P: PacketPayload> Deref for DecodedPacket<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}
