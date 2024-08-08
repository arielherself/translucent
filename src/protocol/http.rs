use crate::{bytes::next_crlf, bytes_formatter, types::{error::ParseError, TranslucentPacket}};

use super::SupportedProtocol;


/// This struct should also recognize forwarded http request.
pub struct HttpProtocol;

impl TryInto<TranslucentPacket> for HttpProtocol {
    type Error = ParseError;

    fn try_into(self) -> Result<TranslucentPacket, Self::Error> {
        todo!()
    }
}

impl HttpProtocol {
    pub fn from_packet(partial_packet: &[u8]) -> Option<Self> {
        let bytes_recv = partial_packet.len();
        bytes_formatter::BytesFormatter::new().print_bytes(partial_packet, bytes_recv);
        let lines = &partial_packet[..bytes_recv];
        if let Some(idx) = next_crlf(lines) {
            let first_line: Vec<&[u8]> = lines[..idx].split(|b| b == &b' ').collect();
            // TODO: support complete spec
            if first_line.len() == 3
               && (first_line[0] == b"GET" || first_line[0] == b"POST")
               && first_line[2] == b"HTTP/1.1" {
                log::debug!("Hit http request.");
                return Some(Self {
                    // TODO:
                });
            }
        }
        None
    }
}

// TODO: implementation
impl SupportedProtocol for HttpProtocol {
    fn exact_size(&self) -> Option<usize> {
        todo!()
    }
    fn extend(&mut self, data: &[u8]) {
        todo!()
    }
    fn wrap(&self) -> Result<TranslucentPacket, Box<ParseError>> {
        todo!()
    }
}
