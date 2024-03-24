use crate::common_traits::device::{Described, PowerConsumptionMeter, SmartDevice, Switchable};
use crate::devices::socket::SocketTrait;

#[derive(Debug)]
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

impl Described for SocketStub {
    fn description(&self) -> String {
        self.description.clone()
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

impl SmartDevice for SocketStub {
    fn as_socket(&self) -> Option<&dyn SocketTrait> {
        Some(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods() {
        let mut kitchen_socket = SocketStub::new("Kitchen".to_string());
        assert_eq!(kitchen_socket.current_state(), false);
        assert_eq!(kitchen_socket.power_consumption_wt(), 0.0);
        assert_eq!(kitchen_socket.turn_on(), true);
        assert_eq!(kitchen_socket.current_state(), true);
        assert_eq!(kitchen_socket.turn_off(), true);
        assert_eq!(kitchen_socket.current_state(), false);
        assert_eq!(kitchen_socket.description, "Kitchen".to_string());
    }
}