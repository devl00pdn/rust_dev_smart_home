use std::io::Error;
use std::net::{ToSocketAddrs, UdpSocket};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crate::common::traits::Described;
use crate::common::traits::device::Err;
use crate::common::traits::device::OptReplay;
use crate::devices::thermometer::TemperatureSensorTrait;

pub struct ThermometerUdp {
    thread_stop: Arc<AtomicBool>,
    thermometer: Arc<Mutex<Thermometer>>,
    // thread_handle: JoinHandle<Result<(), Error>>,
}

impl ThermometerUdp {
    pub fn new<T>(addr: T) -> Result<Self, Error>
        where T: ToSocketAddrs {
        let socket = UdpSocket::bind(addr).map_err(|e| {
            println!("Error. udp socket bind failed");
            e
        })?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        let thread_stop = Arc::new(AtomicBool::default());
        let thread_stop_cloned = thread_stop.clone();
        let thermometer = Arc::new(Mutex::new(Thermometer::default()));
        let thermometer_cloned = thermometer.clone();
        let _ = thread::spawn(move || -> Result<(), Error> {
            loop {
                if thread_stop_cloned.load(Ordering::SeqCst) {
                    return Ok(());
                }
                let mut buf = [0; 255];
                let len = socket.recv(&mut buf)?;
                if len == 0 {
                    continue;
                }
                let mgs_raw = &buf[..len];
                let msgs_vec = protocol::protocol::unwrap_message(std::str::from_utf8(mgs_raw).unwrap()).unwrap_or(vec!["".to_string()]);
                if let Ok(mut thermometer) = thermometer_cloned.lock() {
                    for msg in msgs_vec {
                        if let Ok(temp_c) = f32::from_str(msg.as_str()) {
                            println!("Temperature updated:  {}", msg);
                            thermometer.update_temp_c(temp_c)
                        }
                    }
                }
            }
        });
        Ok(Self { thread_stop, thermometer })
    }
}

impl Drop for ThermometerUdp {
    fn drop(&mut self) {
        self.thread_stop.store(true, Ordering::SeqCst)
    }
}

pub struct Thermometer {
    temp_c: f32,
    is_updated: bool,
}

impl Thermometer {
    pub fn default() -> Thermometer {
        Self { temp_c: 0.0, is_updated: false }
    }

    pub fn update_temp_c(&mut self, new_temp_c: f32) {
        self.temp_c = new_temp_c;
        self.is_updated = true;
    }
}

impl crate::common::traits::device::Thermometer for ThermometerUdp {
    fn temperature_deg_celsius(&self) -> OptReplay<f32> {
        if let Ok(thermometer) = self.thermometer.lock() {
            if thermometer.is_updated == false {
                return Ok(None);
            }
            return Ok(Some(thermometer.temp_c));
        }
        Err(Err { msg: "mutex lock failed".to_string() })
    }
}

impl Described for ThermometerUdp {}

impl TemperatureSensorTrait for ThermometerUdp {}