use std::{thread, time};
use std::error::Error;
use std::net::SocketAddr;

use smart_home_lib::common::traits::Described;
use smart_home_lib::common::traits::device::{PowerConsumptionMeter, Switchable};
use smart_home_lib::devices::socket_tcp::SocketTcp;
use smart_home_lib::devices::thermometer_udp::ThermometerUdp;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Listening smart thermometr over udp...");
    let _thermometer_udp = ThermometerUdp::new("127.0.0.1:34255");
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
