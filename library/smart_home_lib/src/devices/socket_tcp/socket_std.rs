use std::net::ToSocketAddrs;

use protocol::client_std::{ClientStp, RequestResult};
use protocol::errors::ConnectResult;

use crate::common::traits::Described;
use crate::common::traits::device::{OptReplay, PowerConsumptionMeter, Replay, Switchable};
use crate::common::traits::device::ErrorSm;
use crate::devices::socket::SocketTrait;

pub struct SocketTcp {
    client: ClientStp,
}

impl SocketTcp {
    pub fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        Ok(Self { client: ClientStp::connect(addr)? })
    }

    fn handle_result(result: RequestResult) -> Replay<bool> {
        match result {
            Ok(_) => {
                // println!("{}", resp);
                Ok(true)
            }
            Err(err) => {
                // println!("{}", err);
                Err(ErrorSm { msg: err.to_string() })
            }
        }
    }
}

impl PowerConsumptionMeter for SocketTcp {
    fn power_consumption_wt(&mut self) -> OptReplay<f32> {
        let result = self.client.send_request("get_power_consumption_wt");
        match result {
            Ok(val) => { Ok(Some(val.parse::<f32>().unwrap())) }
            Err(err) => {
                println!("{}", err);
                Err(ErrorSm { msg: err.to_string() })
            }
        }
    }
}

impl Switchable for SocketTcp {
    fn turn_on(&mut self) -> Replay<bool> {
        let result = self.client.send_request("turn_on");
        Self::handle_result(result)
    }

    fn turn_off(&mut self) -> Replay<bool> {
        let result = self.client.send_request("turn_off");
        Self::handle_result(result)
    }

    fn current_state(&mut self) -> Replay<bool> {
        match self.client.send_request("get_state") {
            Ok(repl) => {
                Ok(repl.eq("state: on"))
            }
            Err(e) => { Err(ErrorSm { msg: e.to_string() }) }
        }
    }
}

impl Described for SocketTcp {
    fn description(&mut self) -> String {
        let result = self.client.send_request("get_description");
        match result {
            Ok(val) => { val }
            Err(err) => {
                err.to_string()
            }
        }
    }
}

impl SocketTrait for SocketTcp {}

