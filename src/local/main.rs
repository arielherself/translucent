use std::net::Ipv4Addr;

use tokio::{io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::{TcpListener, TcpStream}};

const BUFFER_SIZE: usize = 1024;

fn print_bytes(bytes: &[u8], n: usize) {
    const COLUMN: usize = 16;
    eprintln!("    {}", ansi_term::Colour::Green.bold().paint(format!("Size: {}", n)));
    for byte in bytes[..n].chunks(COLUMN) {
        eprint!("      ");
        for b in byte {
            eprint!("{}", ansi_term::Colour::Cyan.paint(format!("{:02x} ", b)));
        }
        eprint!("{}", String::from_utf8(vec![b' '; 10 + (COLUMN - byte.len()) * 3]).unwrap());
        for b in byte {
            eprint!("{}", if b.is_ascii_graphic() {
                format!("{}", *b as char)
            } else {
                ".".to_string()
            });
        }
        eprintln!();
    }
}

enum Host {
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

struct S5ProxyRelay {
    host: Host,
    port: u16,
    local: tokio::net::TcpStream,
    remote: tokio::net::TcpStream,
}

async fn relay_stream(mut from: ReadHalf<TcpStream>, mut to: WriteHalf<TcpStream>, debug_target: &str) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut buf = [0; BUFFER_SIZE];
        let read_result = from.read(&mut buf).await;
        match read_result {
            Ok(bytes_recv) => {
                // log::debug!(target: debug_target, "Displaying received bytes");
                // print_bytes(&buf, bytes_recv);
                if bytes_recv > 0 {
                    // debug_assert!(!buf[..bytes_recv].contains(&0x00));
                    match to.write_all(&buf[..bytes_recv]).await {
                        Ok(()) => log::debug!(target: debug_target, "Successfully relayed {} bytes", bytes_recv),
                        Err(e) => log::warn!(target: debug_target, "Failed to write to remote: {}", e),
                    }
                } else {
                    log::info!(target: debug_target, "Remote closed connection");
                    to.shutdown().await?;
                    return Ok(());
                }
            },
            Err(e) => {
                log::warn!(target: debug_target, "Failed to read from remote: {}", e);
            }
        }
    }
}

impl S5ProxyRelay {
    async fn from(mut socket: tokio::net::TcpStream) -> Option<Self> {
        let mut buf = [0; BUFFER_SIZE];
        let mut status = 0;
        while let Ok(bytes_recv) = socket.read(&mut buf).await {
            log::debug!("Showing received bytes during initialization");
            print_bytes(&buf, bytes_recv);
            if status == 0 && bytes_recv >= 2 && buf[0] == 0x05 {
                // handshake
                if let Ok(()) = socket.write_all(&[0x05, 0x00]).await {
                    log::info!("Sent 0x05 0x00");
                    status = 1;
                }
            } else if status == 1 && bytes_recv > 3 && buf[..3] == [0x05, 0x01, 0x00] {
                // tcp connect
                log::debug!("buf[4] = {}", buf[3]);
                let (host, port_offset) = match buf[3] {
                    0x01 => (Some(Host::Ipv4(Ipv4Addr::new(buf[4], buf[5], buf[6], buf[7]))), 8),
                    0x03 => (Some(Host::Hostname(buf[5..5 + (buf[4] as usize)].into())), 5 + (buf[4] as usize)),
                    _ => break,  // TODO: support ipv6
                };
                let port = (buf[port_offset] as u16) << 8 | buf[port_offset + 1] as u16;
                if let Ok(()) = socket.write_all(&[&[0x05, 0x00, 0x00, 0x01], &[0u8;6] as &[u8]].concat()).await {
                    log::info!("Successfully received Socks5 proxy request to {}:{}", host.as_ref().unwrap(), port);
                    if let Some(host) = host {
                        if let Ok(stream) = match &host {
                            Host::Ipv4(addr) => TcpStream::connect((addr.to_owned(), port)).await,
                            Host::Hostname(name) => TcpStream::connect((std::str::from_utf8(name).unwrap(), port)).await,
                        } {
                            return Some(Self {
                                host,
                                port,
                                local: socket,
                                remote: stream,
                            });
                        } else {
                            log::warn!("Failed to connect to {}:{}", host, port);
                        }
                    }
                }
            } else {
                break;
            }
        }
        None
    }

    /// Relay a single tcp connection
    async fn decay(mut self) {
        // copy_bidirectional(&mut self.local, &mut self.remote).await;
        let (local_read, local_write) = split(self.local);
        let (remote_read, remote_write) = split(self.remote);
        tokio::spawn(async move {
            let _ = relay_stream(local_read, remote_write, "outbound").await;
        });
        tokio::spawn(async move {
            let _ = relay_stream(remote_read, local_write, "inbound").await;
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:3999").await?;
    log::info!("Listening at 127.0.0.1:3999");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Some(relay) = S5ProxyRelay::from(socket).await {
                relay.decay().await;
            } else {
                log::warn!("Request not initialized.");
            }
        });
    }
}
