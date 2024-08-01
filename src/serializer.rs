use std::error::Error;

use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{bytes::next_crlf, bytes_formatter, consts::BUFFER_SIZE, protocol::{http::HttpProtocol, SupportedProtocol}, types::{error::{ParseError, UnknownError}, TranslucentPacket}};

pub struct StatelessSerializer;

impl StatelessSerializer {
    pub fn serialize<Protocol: SupportedProtocol>(packet: &TranslucentPacket<Protocol>) -> Option<Vec<u8>> {
        // TODO: implementation
        todo!()
    }

    pub fn deserialize(packet: &[u8]) -> Result<TranslucentPacket<impl SupportedProtocol>, ParseError> {
        // TODO: implementation

        if let Some(x) = HttpProtocol::from_packet(packet) {
            // match http
            x.try_into()
        } else {
            Err(ParseError)
        }
    }

    pub async fn deserialize_from_stream(socket: &mut TcpStream) -> Result<TranslucentPacket<impl SupportedProtocol>, Box<dyn Error + Send + Sync>> {
        // TODO: implementation
        let mut buf = [0; BUFFER_SIZE];
        match socket.read(&mut buf).await {
            Ok(bytes_recv) => {
                if let Some(p) = HttpProtocol::from_packet(&buf[..bytes_recv]) {
                    todo!()
                }
            },
            Err(e) => todo!(),
        }
        Err::<TranslucentPacket<crate::protocol::http::HttpProtocol>, Box<dyn Error + Send + Sync>>(Box::new(UnknownError))
    }
}
