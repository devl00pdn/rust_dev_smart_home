use smart_home_derive::Described;

use crate::common::traits::Described;
use crate::common::traits::device::{PowerConsumptionMeter, SmartDevice, Switchable};
use crate::devices::socket::SocketTrait;

#[derive(Debug, Described)]
pub struct SocketStub {
    power_consumption_wt: f32,
    state: bool,
    description: String,
}

impl SocketStub {
    pub fn new(desc: String) -> SocketStub {
        SocketStub { power_consumption_wt: 0.0, state: false, description: desc }
    }
}

impl PowerConsumptionMeter for SocketStub {
    fn power_consumption_wt(&self) -> f32 {
        self.power_consumption_wt
    }
}

impl Switchable for SocketStub {
    fn turn_on(&mut self) -> bool {
        self.state = true;
        println!("Socket turned on");
        true
    }

    fn turn_off(&mut self) -> bool {
        self.state = false;
        println!("Socket turned off");
        true
    }

    fn current_state(&self) -> bool {
        self.state
    }
}

impl SocketTrait for SocketStub {}

impl SmartDevice for SocketStub {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods() {
        let mut kitchen_socket = SocketStub::new("Kitchen".to_string());
        assert!(!kitchen_socket.current_state());
        assert_eq!(kitchen_socket.power_consumption_wt(), 0.0);
        assert!(kitchen_socket.turn_on());
        assert!(kitchen_socket.current_state());
        assert!(kitchen_socket.turn_off());
        assert_eq!(kitchen_socket.description, "Kitchen".to_string());
        assert!(!kitchen_socket.current_state());
    }
}