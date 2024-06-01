use std::{thread, time};
use std::error::Error;
use std::net::SocketAddr;

use devices::socket_tcp::SocketTcp;
use smart_home_lib::common::traits::device::Switchable;

mod devices;

fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:55331".parse()?;
    // let mut client = protocol::client::ClientStp::connect(addr)?;
    let mut socket_tcp = SocketTcp::new(addr)?;

    loop {
        // let resp = client.send_request("Hi socket, im smart home server")?;
        let resp = socket_tcp.turn_on()?;
        println!("socket resp: {}", resp);
        thread::sleep(time::Duration::from_secs(1));
    }
}
