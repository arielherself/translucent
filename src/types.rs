use std::net::Ipv4Addr;

use num_derive::FromPrimitive;

use crate::bytes::parse_usize;

/// Error types for debugging purposes.
pub mod error {
    /// A general connection error. Should have more variants, like relay connection error, error
    /// when connecting remote, ...
    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub struct ConnectionError;

    /// Received data cannot be parsed as a tls packet.
    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub struct ParseError;

    /// All other errors.
    #[derive(Debug, derive_more::Display, derive_more::Error)]
    pub struct UnknownError;
}

/// Two form of target address.
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


pub struct TranslucentPacket {
    pub host: Host,
    pub port: u16,
    pub payload: Vec<u8>,
}

pub trait TLSWrappableType {
    fn from_bytes(data: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn len() -> usize;
}

pub enum TLSVersion {
    Tls12 = 3 << 8 | 3,
}

/// Specification at <https://datatracker.ietf.org/doc/html/rfc5246#section-7.4>
pub enum TLSHandshakeType {
    HelloRequest = 0,
    ClientHello = 1,
    ServerHello = 2,
    Certificate = 11,
    ServerKeyExchange = 12,
    CertificateRequest = 13,
    ServerHelloDone = 14,
    CertificateVerify = 15,
    ClientKeyExchange = 16,
    Finished = 20,
    Unknown = 255,
}

pub enum TLSContentType {
    ChangeCipherSpec = 20,
    Alert = 21,
    Handshake = 22,
    ApplicationData = 23,
    Unknown = 255,
}

pub struct TLSHandshake {
    msg_type: TLSHandshakeType,
    length: u32,
    body: Vec<u8>,
}

pub struct TLSPlainText {
    content_type: TLSContentType,
    protocol_version: TLSVersion,
    length: u16,
    fragment: Vec<u8>,
}

pub struct TLSRandom {
    gmt_unix_time: u32,
    random_bytes: [u8; 28],
}

#[derive(FromPrimitive)]
pub enum TLSCompressionMethod {
    Null = 0,
    Unknown = 255,
}

impl TLSWrappableType for TLSCompressionMethod {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Null => vec![0],
            Self::Unknown => vec![255],
        }
    }
    fn len() -> usize {
        1
    }
    fn from_bytes(data: &[u8]) -> Self {
        debug_assert_eq!(data.len(), 1);
        num::FromPrimitive::from_u8(data[0]).unwrap()
    }
}

pub struct TLSVector<T: TLSWrappableType> {
    length: usize,
    data: Vec<T>,
}

impl<T: TLSWrappableType> TLSVector<T> {
    pub fn from_bytes(data: &[u8], max_len: usize) -> Self {
        let length_len = ((usize::BITS - max_len.leading_zeros() + 7) / 8) as usize;
        Self {
            length: parse_usize(&data[..length_len]).unwrap(),
            data: data[length_len..].chunks(T::len()).map(|x| T::from_bytes(x)).collect()
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn get(&self, i: usize) -> &T {
        &self.data[i]
    }
}

pub struct TLSClientHelloMessage {
    client_version: TLSVersion,
    random: TLSRandom,
    session_id: [u8; 32],
    cipher_suite: [u8; 2],  // TODO: vector
    compression_methods: TLSVector<TLSCompressionMethod>,
    extensions: TLSVector<TLSExtension>,  // TODO: enum
}
