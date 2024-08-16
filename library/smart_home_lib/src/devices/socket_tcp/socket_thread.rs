use std::io::{Error, ErrorKind};
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::common::traits::Described;
use crate::common::traits::device::{ErrorSm, OptReplay, PowerConsumptionMeter, Replay, Switchable};
use crate::devices::socket::SocketTrait;
use crate::devices::socket_tcp::socket_std::SocketTcp;

pub struct SocketTcpWrapper {
    thread_stop: Arc<AtomicBool>,
    socket: Arc<Mutex<(SocketTcp, SocketData)>>,
}


impl SocketTcpWrapper {
    pub fn new<T>(addr: T, update_period: Duration) -> Result<Self, Error>
    where
        T: ToSocketAddrs,
    {
        let thread_stop = Arc::new(AtomicBool::default());
        let thread_stop_cloned = thread_stop.clone();
        let socket_tcp = SocketTcp::new(addr).map_err(|_| Error::new(ErrorKind::Other, "connection error"))?;

        let socket = Arc::new(Mutex::new((socket_tcp, SocketData::default())));
        let socket_cloned = socket.clone();
        let _ = thread::spawn(move || -> Result<(), ErrorSm> {
            loop {
                if thread_stop_cloned.load(Ordering::SeqCst) {
                    return Ok(());
                }
                sleep(update_period);
                if let Ok(mut socket) = socket_cloned.lock() {
                    socket.1.last_received_state = socket.0.current_state()?;
                    socket.1.last_received_pwr = socket.0.power_consumption_wt()?;
                }
            }
        });
        Ok(Self { thread_stop, socket })
    }
}

impl Drop for SocketTcpWrapper {
    fn drop(&mut self) {
        self.thread_stop.store(true, Ordering::SeqCst)
    }
}
#[derive(Default)]
struct SocketData {
    last_received_state: bool,
    last_received_pwr: Option<f32>,
}

impl PowerConsumptionMeter for SocketTcpWrapper {
    fn power_consumption_wt(&mut self) -> OptReplay<f32> {
        if let Ok(socket) = self.socket.lock() {
            return Ok(socket.1.last_received_pwr);
        }
        Ok(None)
    }
}

impl Switchable for SocketTcpWrapper {
    fn turn_on(&mut self) -> Replay<bool> {
        if let Ok(mut socket) = self.socket.lock() {
            return socket.0.turn_on();
        }
        Err(ErrorSm { msg: "lock failed".to_string() })
    }

    fn turn_off(&mut self) -> Replay<bool> {
        if let Ok(mut socket) = self.socket.lock() {
            return socket.0.turn_off();
        }
        Err(ErrorSm { msg: "lock failed".to_string() })
    }

    fn current_state(&mut self) -> Replay<bool> {
        if let Ok(socket) = self.socket.lock() {
            return Ok(socket.1.last_received_state);
        }
        Err(ErrorSm { msg: "lock failed".to_string() })
    }
}

impl Described for SocketTcpWrapper {}

impl SocketTrait for SocketTcpWrapper {}
