use std::net::Ipv4Addr;

use crate::protocol::SupportedProtocol;

pub mod error {
    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub struct ConnectionError;

    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub struct ParseError;

    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub struct UnknownError;
}

#[derive(Clone)]
pub enum Host {
    Ipv4(Ipv4Addr),
    Hostname(Vec<u8>),
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Host::Ipv4(addr) => write!(f, "{}", addr),
            Host::Hostname(name) => write!(f, "{}", std::str::from_utf8(name).unwrap()),
        }
    }
}

pub struct TranslucentPacket<Protocol: SupportedProtocol> {
    pub host: Host,
    pub port: u16,
    pub payload: Protocol::PayloadType,
}
