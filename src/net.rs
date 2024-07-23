use std::error::Error;

use tokio::net::TcpStream;

use crate::types::Host;

pub async fn connect(host: &Host, port: u16) -> Result<TcpStream, Box<dyn Error + Send>> {
    match match host {
        Host::Ipv4(addr) => TcpStream::connect((addr.to_owned(), port)).await,
        Host::Hostname(name) => TcpStream::connect((std::str::from_utf8(name).unwrap(), port)).await,
    } {
        Ok(stream) => Ok(stream),
        Err(e) => Err(Box::new(e)),
    }
}
