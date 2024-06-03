use std::{io, thread, time};
use std::error::Error;
use std::net::{SocketAddr, UdpSocket};

use devices::socket_tcp::SocketTcp;
use smart_home_lib::common::traits::Described;
use smart_home_lib::common::traits::device::{PowerConsumptionMeter, Switchable};

mod devices;


fn thermo_udp_listener() -> Result<(), io::Error> {
    let socket = UdpSocket::bind("127.0.0.1:34254").map_err(|e| {
        println!("Error. udp socket bind failed");
        e
    })?;
    println!("Receiving temp from udp thermometer 5 times...");
    let mut exit_counter = 0;
    loop {
        let mut buf = [0; 255];
        let len = socket.recv(&mut buf)?;
        if len == 0 {
            continue;
        }
        let mgs_raw = &buf[..len];
        let msgs_vec = protocol::protocol::unwrap_message(std::str::from_utf8(mgs_raw).unwrap()).unwrap_or(vec!["".to_string()]);
        for msg in msgs_vec {
            println!("Received temperature: {}", msg);
        }
        exit_counter += 1;
        if exit_counter >= 5 {
            return Ok(());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = thread::spawn(move || -> Result<(), io::Error> { thermo_udp_listener() }).join().map_err(|_| println!("Something goes wrong.."));

    println!("Connecting to smart socket over tcp...");

    let addr: SocketAddr = "127.0.0.1:55331".parse()?;

    let mut socket_tcp = SocketTcp::new(addr)?;

    println!("> Request socket description");
    let resp = socket_tcp.description();
    println!("socket description: {}", resp);

    println!("> turn on socket");
    let _ = socket_tcp.turn_on()?;

    thread::sleep(time::Duration::from_secs(1));

    println!("> Request socket state");
    let _ = socket_tcp.current_state()?;

    println!("> Request socket power consumption");
    match socket_tcp.power_consumption_wt()? {
        None => {
            println!("socket power consumption wt: unknown");
        }
        Some(resp) => {
            println!("socket power consumption wt: {}", resp);
        }
    }

    thread::sleep(time::Duration::from_secs(1));

    println!("> turn off socket");
    let _ = socket_tcp.turn_off()?;


    println!("> Request socket state");
    let _ = socket_tcp.current_state()?;

    thread::sleep(time::Duration::from_secs(1));
    Ok(())
}
