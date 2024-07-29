use crate::types::{error::ParseError, TranslucentPacket};

use super::SupportedProtocol;

pub struct HttpPayload {
    pub size: usize,
    pub header: Vec<u8>,
    pub content: Vec<u8>,
}


pub struct HttpProtocol;

impl TryInto<TranslucentPacket<HttpProtocol>> for HttpProtocol {
    type Error = ParseError;

    fn try_into(self) -> Result<TranslucentPacket<HttpProtocol>, Self::Error> {
        todo!()
    }
}

// TODO: implementation
impl SupportedProtocol for HttpProtocol {
    type PayloadType = HttpPayload;
    fn from(packet: &[u8]) -> Option<Self> {
        todo!()
    }
    fn exact_size() -> Option<usize> {
        todo!()
    }
}
