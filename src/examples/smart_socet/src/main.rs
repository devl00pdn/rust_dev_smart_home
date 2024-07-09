use std::error::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;

use protocol::client_std::{RequestError, RequestResult};
use protocol::errors::RecvError;
use protocol::server_std;
use protocol::server_std::StpConnection;
use smart_home_lib::common::types::SmartPointer;
use smart_home_lib::devices::socket::SocketTrait;
use smart_home_lib::devices::stubs::socket_stub::SocketStub;

fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:55331".parse()?;
    println!("SmartSocket server_tcp running at addr {}", addr);
    let socket_stub = SocketStub::new("Kitchen socket via tcp".to_string());
    let server = server_std::ServerStp::bind(addr)?;
    for connection_res in server.incoming() {
        let mut connection = connection_res?;
        println!("client connected from: {}", connection.peer_addr().unwrap());
        loop {
            match process(&mut connection, socket_stub.clone()) {
                Ok(_) => {}
                Err(RequestError::Recv(RecvError::Io(e))) if e.kind() == ErrorKind::BrokenPipe => {
                    println!("client {} disconnected", connection.peer_addr().unwrap());
                    break;
                }
                Err(e) => { println!("Error: {}", e) }
            }
        }
    }
    Ok(())
}

fn process<Socket>(conn: &mut StpConnection, socket: SmartPointer<Socket>) -> RequestResult
where
    Socket: SocketTrait,
{
    let req = conn.revc_request()?;
    println!("{}", req);

    let resp: String = match req.as_str() {
        "turn_on" => {
            match socket.borrow_mut().turn_on() {
                Ok(_) => { "ok".to_string() }
                Err(e) => { e.to_string() }
            }
        }
        "turn_off" => {
            match socket.borrow_mut().turn_off() {
                Ok(_) => { "ok".to_string() }
                Err(e) => { e.to_string() }
            }
        }
        "get_state" => {
            match socket.borrow_mut().current_state() {
                Ok(state) => { if state { "state: on".to_string() } else { "state: off".to_string() } }
                Err(e) => { e.to_string() }
            }
        }
        "get_power_consumption_wt" => {
            match socket.borrow_mut().power_consumption_wt() {
                Ok(Some(pwr)) => { format!("{}", pwr) }
                Err(e) => { e.to_string() }
                _ => { "Unknown power_consumption".to_string() }
            }
        }
        "get_description" => {
            socket.borrow_mut().description()
        }
        _ => { "Unknown request".to_string() }
    };
    conn.send_response(resp)?;
    Ok("".to_string())
}
