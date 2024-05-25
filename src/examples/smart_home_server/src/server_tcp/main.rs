use std::{thread, time};
use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:55331".parse()?;
    let mut client = protocol::client::ClientStp::connect(addr)?;
    loop {
        let resp = client.send_request("Hi socket, im smart home server")?;
        println!("socket resp: {}", resp);
        thread::sleep(time::Duration::from_secs(1));
    }
}
