use std::{error::Error, net::Ipv4Addr};

#[derive(Debug)]
pub struct ConnectionError(pub String);

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ConnectionError {}

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

pub struct HttpPacket {
    pub size: usize,
    pub header: Vec<u8>,
    pub content: Vec<u8>,
}

pub enum TranslucentPayload {
    Bytes(Vec<u8>),
    Http(HttpPacket),
    // TODO: Tls(TlsPacket),
}

pub struct TranslucentPacket {
    pub host: Host,
    pub port: u16,
    pub payload: TranslucentPayload,
}
