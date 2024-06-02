use std::cell::RefCell;
use std::rc::Rc;

use smart_home_derive::Described;

use crate::common::traits::Described;
use crate::common::traits::device::{Err, OptReplay, PowerConsumptionMeter, Replay, SmartDevice, Switchable};
use crate::common::types::SmartPointer;
use crate::devices::socket::SocketTrait;

#[derive(Debug, Described)]
pub struct SocketStub {
    power_consumption_wt: f32,
    state: bool,
    description: String,
    /// true - device online
    connection_state_emulation: bool,
}

impl SocketStub {
    pub fn new(desc: String) -> SmartPointer<SocketStub> {
        Rc::new(RefCell::new(SocketStub { power_consumption_wt: 0.0, state: false, description: desc, connection_state_emulation: true }))
    }

    pub fn online(&mut self, state: bool) {
        self.connection_state_emulation = state
    }
}

impl PowerConsumptionMeter for SocketStub {
    fn power_consumption_wt(&mut self) -> OptReplay<f32> {
        if !self.connection_state_emulation {
            return Err(Err { msg: "Device not respond".to_string() });
        }
        Ok(Some(self.power_consumption_wt))
    }
}

impl Switchable for SocketStub {
    fn turn_on(&mut self) -> Replay<bool> {
        self.state = true;
        if !self.connection_state_emulation {
            return Err(Err { msg: "Device not respond".to_string() });
        }
        self.power_consumption_wt = 2000.;
        Ok(true)
    }

    fn turn_off(&mut self) -> Replay<bool> {
        self.state = false;
        if !self.connection_state_emulation {
            return Err(Err { msg: "Device not respond".to_string() });
        }
        self.power_consumption_wt = 0.;
        Ok(true)
    }

    fn current_state(&mut self) -> Replay<bool> {
        if !self.connection_state_emulation {
            return Err(Err { msg: "Device not respond".to_string() });
        }
        Ok(self.state)
    }
}

impl SocketTrait for SocketStub {}

impl SmartDevice for SocketStub {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods() {
        let kitchen_socket = SocketStub::new("Kitchen".to_string());
        assert!(!kitchen_socket.borrow_mut().current_state().unwrap());
        assert_eq!(kitchen_socket.borrow_mut().power_consumption_wt().unwrap().unwrap(), 0.0);
        assert!(kitchen_socket.borrow_mut().turn_on().unwrap());
        assert!(kitchen_socket.borrow_mut().current_state().unwrap());
        assert!(kitchen_socket.borrow_mut().turn_off().unwrap());
        assert_eq!(kitchen_socket.borrow_mut().description, "Kitchen".to_string());
        assert!(!kitchen_socket.borrow_mut().current_state().unwrap());
        kitchen_socket.borrow_mut().online(false);
        assert!(kitchen_socket.borrow_mut().current_state().is_err());
        assert!(kitchen_socket.borrow_mut().turn_on().is_err());
        assert!(kitchen_socket.borrow_mut().turn_off().is_err());
        kitchen_socket.borrow_mut().online(true);
    }
}