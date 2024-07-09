use std::{thread, time};
use std::error::Error;
use std::net::UdpSocket;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    let start = Instant::now();

    println!("Sending simulated temp over udp to 127.0.0.1:34255...");
    loop {
        let duration = start.elapsed();
        let simulated_temp_deg = 25.0 + 10.0 * (duration.as_secs() as f32 * 0.3).sin();
        let msg = protocol::protocol::wrap_message(format!("{}", simulated_temp_deg));
        socket.send_to(msg.as_ref(), "127.0.0.1:34255")?;
        thread::sleep(time::Duration::from_millis(50));
    }
}
