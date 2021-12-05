use std::net::{TcpListener};
use ruster::client;
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        client::handle_client(stream?);
    }
    Ok(())
}
