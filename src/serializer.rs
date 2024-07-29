use std::error::Error;

use tokio::net::TcpStream;

use crate::{protocol::SupportedProtocol, types::{TranslucentPacket, error::UnknownError}};

pub struct StatelessSerializer;

impl StatelessSerializer {
    pub fn serialize<Protocol: SupportedProtocol>(packet: &TranslucentPacket<Protocol>) -> Option<Vec<u8>> {
        // TODO: implementation
        todo!()
    }

    pub fn deserialize(packet: &[u8]) -> Option<TranslucentPacket<impl SupportedProtocol>> {
        // TODO: implementation
        None as Option<TranslucentPacket<crate::protocol::http::HttpProtocol>>
    }

    pub async fn deserialize_from_stream(socket: &mut TcpStream) -> Result<TranslucentPacket<impl SupportedProtocol>, Box<dyn Error + Send + Sync>> {
        // TODO: implementation
        Err::<TranslucentPacket<crate::protocol::http::HttpProtocol>, Box<dyn Error + Send + Sync>>(Box::new(UnknownError))
    }
}
