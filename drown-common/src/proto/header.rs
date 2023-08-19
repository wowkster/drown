#[derive(Debug, Default, Clone)]
pub struct PacketHeader {
    pub message_id: u32,
    pub response_to: Option<u32>,
}

impl PacketHeader {
    pub const SIZE: usize = 8;

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), Self::SIZE);

        Self {
            message_id: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            response_to: match u32::from_le_bytes(bytes[4..8].try_into().unwrap()) {
                0 => None,
                x => Some(x),
            },
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::SIZE);

        bytes.extend_from_slice(&self.message_id.to_le_bytes());
        bytes.extend_from_slice(&self.response_to.map_or(0u32, |x| x).to_le_bytes());

        bytes
    }
}
