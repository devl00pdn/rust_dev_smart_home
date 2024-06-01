use std::net::ToSocketAddrs;

use protocol::client::ClientStp;
use protocol::errors::ConnectResult;
use smart_home_lib::common::traits::Described;
use smart_home_lib::common::traits::device::{OptReplay, PowerConsumptionMeter, Replay, Switchable};
use smart_home_lib::common::traits::device::Err;
use smart_home_lib::devices::socket::SocketTrait;

pub struct SocketTcp {
    client: ClientStp,
}

impl SocketTcp {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        Ok(Self { client: ClientStp::connect(addr)? })
    }
}

impl PowerConsumptionMeter for SocketTcp {
    fn power_consumption_wt(&self) -> OptReplay<f32> {
        todo!()
    }
}

impl Switchable for SocketTcp {
    fn turn_on(&mut self) -> Replay<bool> {
        match self.client.send_request("turn_on".to_string()) {
            Ok(resp) => {
                println!("{}", resp);
                Ok(true)
            }
            Err(err) => {
                println!("{}", err);
                Err(Err { msg: err.to_string() })
            }
        }
    }

    fn turn_off(&mut self) -> Replay<bool> {
        todo!()
    }

    fn current_state(&self) -> Replay<bool> {
        todo!()
    }
}

impl Described for SocketTcp {
    fn description(&self) -> String {
        todo!()
    }
}

impl SocketTrait for SocketTcp {}

