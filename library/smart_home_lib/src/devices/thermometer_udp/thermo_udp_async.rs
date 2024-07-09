use std::io::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use async_trait::async_trait;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::sync::Mutex;

use crate::common::traits::device::OptReplay;
use crate::common::traits_async::Described;
use crate::devices::thermometer::TemperatureSensorTraitAsync;

pub struct ThermometerUdp {
    thread_stop: Arc<AtomicBool>,
    thermometer: Arc<Mutex<Thermometer>>,
    handle: tokio::task::JoinHandle<Result<(), Error>>,
}

impl ThermometerUdp {
    pub async fn new<T>(addr: T) -> Result<Self, Error>
    where
        T: ToSocketAddrs,
    {
        let socket = UdpSocket::bind(addr).await.map_err(|e| {
            println!("Error. udp socket bind failed");
            e
        })?;
        let thread_stop = Arc::new(AtomicBool::default());
        let thread_stop_cloned = thread_stop.clone();
        let thermometer = Arc::new(Mutex::new(Thermometer::new()));
        let thermometer_cloned = thermometer.clone();
        let handle = tokio::spawn(async move {
            loop {
                if thread_stop_cloned.load(Ordering::SeqCst) {
                    return Ok(());
                }
                let mut buf = [0; 255];
                let len = socket.recv(&mut buf).await?;
                if len == 0 {
                    continue;
                }
                let mgs_raw = &buf[..len];
                let msgs_vec = protocol::protocol::unwrap_message(std::str::from_utf8(mgs_raw).unwrap()).unwrap_or(vec!["".to_string()]);
                let mut thermometer = thermometer_cloned.lock().await;
                for msg in msgs_vec {
                    if let Ok(temp_c) = f32::from_str(msg.as_str()) {
                        thermometer.update_temp_c(temp_c)
                    }
                }
            }
        });
        Ok(Self { thread_stop, thermometer, handle })
    }
}

impl Drop for ThermometerUdp {
    fn drop(&mut self) {
        self.thread_stop.store(true, Ordering::SeqCst);
        self.handle.abort();
    }
}

pub struct Thermometer {
    temp_c: f32,
    is_updated: bool,
}

impl Default for Thermometer {
    fn default() -> Self {
        Thermometer::new()
    }
}

impl Thermometer {
    pub fn new() -> Thermometer {
        Self { temp_c: 0.0, is_updated: false }
    }

    pub fn update_temp_c(&mut self, new_temp_c: f32) {
        self.temp_c = new_temp_c;
        self.is_updated = true;
    }
}

#[async_trait]
impl crate::common::traits_async::device::Thermometer for ThermometerUdp {
    async fn temperature_deg_celsius(&self) -> OptReplay<f32> {
        let thermometer = self.thermometer.lock().await;
        if !thermometer.is_updated {
            return Ok(None);
        }
        return Ok(Some(thermometer.temp_c));
    }
}

impl Described for ThermometerUdp {}

impl TemperatureSensorTraitAsync for ThermometerUdp {}