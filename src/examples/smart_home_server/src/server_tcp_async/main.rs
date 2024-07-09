use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;

use tokio::time::sleep;

use smart_home_lib::common::traits_async::Described;
use smart_home_lib::common::traits_async::device::{PowerConsumptionMeter, Switchable, Thermometer};
use smart_home_lib::devices::socket_tcp::socket_tokio::SocketTcp;
use smart_home_lib::devices::thermometer_udp::thermo_udp_async::ThermometerUdp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Listening smart thermometr over udp...");
    sleep(Duration::from_millis(500)).await;
    println!("Connecting to smart socket over tcp...");
    let addr: SocketAddr = "127.0.0.1:55331".parse()?;
    let (thermometer_udp, socket_tcp) = tokio::join!(ThermometerUdp::new("127.0.0.1:34255"), SocketTcp::new(addr));
    let (thermometer_udp, mut socket_tcp) = (thermometer_udp?, socket_tcp?);
    println!("> Request socket description");
    let resp = socket_tcp.description().await;
    println!("socket description: {}", resp);
    println!("> turn on socket");
    _ = socket_tcp.turn_on().await?;

    sleep(Duration::from_millis(500)).await;
    println!("> Request socket state");
    let (temp_c, _) = tokio::join!(thermometer_udp.temperature_deg_celsius(), socket_tcp.current_state());
    println!("Current temperature:  {}", temp_c?.unwrap_or(0.0));

    sleep(Duration::from_millis(500)).await;
    println!("> Request socket power consumption");
    let (pwr, temp) = tokio::join!(socket_tcp.power_consumption_wt(), thermometer_udp.temperature_deg_celsius());
    match pwr? {
        None => {
            println!("socket power consumption wt: unknown");
        }
        Some(resp) => {
            println!("socket power consumption wt: {}", resp);
        }
    }
    println!("Current temperature:  {}", temp?.unwrap_or(0.0));
    println!("> turn off socket");
    let _ = socket_tcp.turn_off().await?;
    sleep(Duration::from_millis(500)).await;
    println!("> Request socket state");
    let (_, temp) = tokio::join!(socket_tcp.current_state(), thermometer_udp.temperature_deg_celsius());
    println!("Current temperature:  {}", temp?.unwrap_or(0.0));
    Ok(())
}
