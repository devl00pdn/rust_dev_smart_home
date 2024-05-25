use std::error::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;

use protocol::client::{RequestError, RequestResult};
use protocol::errors::RecvError;
use protocol::server;
use protocol::server::StpConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:55331".parse()?;
    println!("SmartSocket server_tcp running at addr {}", addr.to_string());

    let server = server::ServerStp::bind(addr)?;
    for connection_res in server.incoming() {
        let mut connection = connection_res?;
        println!("client connected from: {}", connection.peer_addr().unwrap());
        loop {
            match process(&mut connection) {
                Ok(_) => {}
                Err(RequestError::Recv(RecvError::Io(e))) if e.kind() == ErrorKind::BrokenPipe => {
                    println!("client {} disconnected", connection.peer_addr().unwrap());
                    break;
                }
                Err(_) => {}
            }
        }
    }
    Ok(())
}

fn process(conn: &mut StpConnection) -> RequestResult {
    conn.take_error().expect("No error was expected...");
    let req = conn.revc_request()?;
    println!("{}", req);
    conn.send_response("Ok".to_string())?;
    Ok("".to_string())
}
