use std::{error::Error, sync::Arc};

use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, net::{TcpListener, TcpStream}, sync::Mutex};
use translucent::{bytes_formatter::BytesFormatter, consts::BUFFER_SIZE, net::connect, protocol::SupportedProtocol, serializer::StatelessSerializer, types::{ConnectionError, Host, TranslucentPacket}};

struct TranslucentRelay<Protocol: SupportedProtocol> {
    host: Host,
    port: u16,
    client: TcpStream,
    remote: TcpStream,
    init_packet: Option<TranslucentPacket<Protocol>>,
}

impl <Protocol: SupportedProtocol> TranslucentRelay<Protocol> {
    async fn decay(mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }
}

async fn init_relay(mut socket: TcpStream, bytes_formatter: Arc<Mutex<BytesFormatter>>) -> Result<TranslucentRelay<impl SupportedProtocol>, Box<dyn Error + Send + Sync>> {
    if let Ok(packet) =  StatelessSerializer::deserialize_from_stream(&mut socket).await {
        let remote_stream = connect(&packet.host, packet.port).await?;
        Ok(TranslucentRelay {
            host: packet.host.to_owned(),
            port: packet.port,
            client: socket,
            remote: remote_stream,
            init_packet: Some(packet),
        })
    } else {
        Err(Box::new(ConnectionError))
    }
}

// TODO: This function should be shared between `tllocal` and `tlserver`, which receives a handler
// for serializing/deserializing data.
async fn relay_stream(mut from: ReadHalf<TcpStream>, mut to: WriteHalf<TcpStream>, debug_target: &str, debug_host: &Host, debug_port: u16) -> Result<(), Box<dyn Error>> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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
            if let Ok(relay) = init_relay(socket, bytes_formatter_clone).await {
                match relay.decay().await {
                    Ok(()) => todo!(),
                    Err(e) => todo!(),
                }
            }
        });
    }
}
