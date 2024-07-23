use std::error::Error;

use tokio::net::TcpStream;

use crate::types::TranslucentPacket;

pub struct StatelessSerializer;

impl StatelessSerializer {
    pub fn serialize(packet: &TranslucentPacket) -> Option<Vec<u8>> {
        None
    }

    pub fn deserialize(packet: &[u8]) -> Option<TranslucentPacket> {
        None
    }

    pub async fn deserialize_from_stream(socket: &mut TcpStream) -> Result<TranslucentPacket, Box<dyn Error>> {
        panic!("Call of unimplemented function.");
    }
}
