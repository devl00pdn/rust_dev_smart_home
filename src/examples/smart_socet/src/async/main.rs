use std::error::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::ops::DerefMut;
use std::sync::Arc;

use tokio::sync::Mutex;

use protocol::client_std::{RequestError, RequestResult};
use protocol::errors::RecvError;
use protocol::server_tokio::{ServerStp, StpConnection};
use smart_home_lib::devices::socket::SocketTrait;
use smart_home_lib::devices::stubs::socket_stub::SocketStub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:55331".parse()?;
    println!("SmartSocket server_tcp running at addr {}", addr);
    let socket_stub = SocketStub::new_with_wrap(
        "Kitchen socket via tcp".to_string(),
        |x| { Arc::new(Mutex::new(x)) },
    );

    let server = ServerStp::bind(addr).await?;
    loop {
        let mut connection = server.incoming().await?;
        let socket = socket_stub.clone();
        println!("client connected from: {}", connection.peer_addr().unwrap());
        tokio::spawn(async move {
            loop {
                let mut locked_socket = socket.lock().await;
                match process(&mut connection, locked_socket.deref_mut()).await {
                    Ok(_) => {}
                    Err(RequestError::Recv(RecvError::Io(e))) if e.kind() == ErrorKind::BrokenPipe => {
                        println!("client {} disconnected", connection.peer_addr().unwrap());
                        break;
                    }
                    Err(e) => { println!("Error: {}", e) }
                }
            }
        });
    }
}

async fn process<Socket>(conn: &mut StpConnection, socket: &mut Socket) -> RequestResult
where
    Socket: SocketTrait,
{
    let req = conn.revc_request().await?;
    println!("{}", req);

    let resp: String = match req.as_str() {
        "turn_on" => {
            match socket.turn_on() {
                Ok(_) => { "ok".to_string() }
                Err(e) => { e.to_string() }
            }
        }
        "turn_off" => {
            match socket.turn_off() {
                Ok(_) => { "ok".to_string() }
                Err(e) => { e.to_string() }
            }
        }
        "get_state" => {
            match socket.current_state() {
                Ok(state) => { if state { "state: on".to_string() } else { "state: off".to_string() } }
                Err(e) => { e.to_string() }
            }
        }
        "get_power_consumption_wt" => {
            match socket.power_consumption_wt() {
                Ok(Some(pwr)) => { format!("{}", pwr) }
                Err(e) => { e.to_string() }
                _ => { "Unknown power_consumption".to_string() }
            }
        }
        "get_description" => {
            socket.description()
        }
        _ => { "Unknown request".to_string() }
    };
    conn.send_response(resp).await?;
    Ok("".to_string())
}
