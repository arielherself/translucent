use std::{net::Ipv4Addr, sync::Arc};

use tokio::{io::{split, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::{TcpListener, TcpStream}, sync::Mutex};

const BUFFER_SIZE: usize = 1024;

struct BytesFormatter;

impl BytesFormatter {
    fn new() -> Self {
        Self {}
    }
    fn print_bytes(&self, bytes: &[u8], n: usize) {
        const COLUMN: usize = 16;
        eprintln!("    {}", ansi_term::Colour::Green.bold().paint(format!("Size: {}", n)));
        for byte_row in bytes[..n].chunks(COLUMN) {
            eprint!("      ");
            for b in byte_row {
                eprint!("{}", ansi_term::Colour::Cyan.paint(format!("{:02x} ", b)));
            }
            eprint!("{}", String::from_utf8(vec![b' '; 10 + (COLUMN - byte_row.len()) * 3]).unwrap());
            for b in byte_row {
                eprint!("{}", if b.is_ascii_graphic() {
                    format!("{}", *b as char)
                } else {
                    ".".to_string()
                });
            }
            eprintln!();
        }
    }
}

#[derive(Clone)]
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

async fn relay_stream(mut from: ReadHalf<TcpStream>, mut to: WriteHalf<TcpStream>, debug_target: &str, debug_host: &Host, debug_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; BUFFER_SIZE];
    let debug_t = format!("{}({}:{})", debug_target, debug_host, debug_port);
    loop {
        let read_result = from.read(&mut buf).await;
        match read_result {
            Ok(bytes_recv) => {
                // log::debug!(target: debug_target, "Displaying received bytes");
                // print_bytes(&buf, bytes_recv);
                if bytes_recv > 0 {
                    match to.write_all(&buf[..bytes_recv]).await {
                        Ok(()) => log::debug!(target: &debug_t, "Successfully relayed {} bytes", bytes_recv),
                        Err(e) => log::warn!(target: &debug_t, "Failed to write to remote: {}", e),
                    }
                } else {
                    log::info!(target: &debug_t, "Remote closed connection");
                    to.shutdown().await?;
                    return Ok(());
                }
            },
            Err(e) => {
                log::warn!(target: &debug_t, "Failed to read from remote: {}", e);
            }
        }
    }
}

impl S5ProxyRelay {
    async fn from(mut socket: tokio::net::TcpStream, bytes_formatter: Arc<Mutex<BytesFormatter>>) -> Option<Self> {
        let mut buf = [0; BUFFER_SIZE];
        let mut sub_negotiate_finished = false;
        while let Ok(bytes_recv) = socket.read(&mut buf).await {
            log::debug!("Showing received bytes during initialization");
            bytes_formatter.lock().await.print_bytes(&buf, bytes_recv);
            if !sub_negotiate_finished && bytes_recv >= 2 && buf[0] == 0x05 {
                // handshake
                if let Ok(()) = socket.write_all(&[0x05, 0x00]).await {
                    log::debug!("Sent 0x05 0x00");
                    sub_negotiate_finished = true;
                }
            } else if sub_negotiate_finished && bytes_recv > 3 && buf[..3] == [0x05, 0x01, 0x00] {
                // tcp connect
                let (host, port_offset) = match buf[3] {
                    0x01 => (Some(Host::Ipv4(Ipv4Addr::new(buf[4], buf[5], buf[6], buf[7]))), 8),
                    0x03 => (Some(Host::Hostname(buf[5..5 + (buf[4] as usize)].into())), 5 + (buf[4] as usize)),
                    _ => break,  // TODO: support ipv6
                };
                let port = (buf[port_offset] as u16) << 8 | buf[port_offset + 1] as u16;
                if let Ok(()) = socket.write_all(&[&[0x05, 0x00, 0x00, 0x01], &[0u8; 6] as &[u8]].concat()).await {
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
    async fn decay(self) {
        // copy_bidirectional(&mut self.local, &mut self.remote).await;
        let (local_read, local_write) = split(self.local);
        let (remote_read, remote_write) = split(self.remote);
        let host = self.host.to_owned();
        tokio::spawn(async move {
            let _ = relay_stream(local_read, remote_write, "outbound", &host, self.port).await;
        });
        tokio::spawn(async move {
            let _ = relay_stream(remote_read, local_write, "inbound", &self.host, self.port).await;
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simple_logger::init_with_level(if cfg!(feature = "debug") {
        log::Level::Debug
    } else {
        log::Level::Info
    }).unwrap();
    let bytes_formatter = Arc::new(Mutex::new(BytesFormatter::new()));
    let listener = TcpListener::bind("127.0.0.1:3999").await?;
    log::info!("Listening at 127.0.0.1:3999");

    loop {
        let (socket, _) = listener.accept().await?;
        let bytes_formatter_clone = bytes_formatter.clone();
        tokio::spawn(async move {
            if let Some(relay) = S5ProxyRelay::from(socket, bytes_formatter_clone).await {
                relay.decay().await;
            } else {
                log::warn!("Request not initialized.");
            }
        });
    }
}
