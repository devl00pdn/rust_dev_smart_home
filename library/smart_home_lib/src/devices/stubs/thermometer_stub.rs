use std::cell::RefCell;
use std::rc::Rc;

use smart_home_derive::Described;

use crate::common::traits::Described;
use crate::common::traits::device::{OptReplay, SmartDevice, Thermometer};
use crate::common::types::SmartPointer;
use crate::devices::thermometer::TemperatureSensorTrait;

#[derive(Debug, Described)]
pub struct ThermometerStub {
    description: String,
    current_temp_deg: f32,
    connection_state_emulation: bool,
}

impl Thermometer for ThermometerStub {
    fn temperature_deg_celsius(&self) -> OptReplay<f32> {
        if !self.connection_state_emulation {
            return Err(crate::common::traits::device::Err { msg: "Device not respond".to_string() });
        }
        Ok(Some(self.current_temp_deg))
    }
}

impl ThermometerStub {
    pub fn new(description: String) -> SmartPointer<ThermometerStub> {
        Rc::new(RefCell::new(ThermometerStub { description, current_temp_deg: 0.0, connection_state_emulation: true }))
    }

    pub fn online(&mut self, state: bool) {
        self.connection_state_emulation = state
    }
}

impl TemperatureSensorTrait for ThermometerStub {}

impl SmartDevice for ThermometerStub {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods() {
        let term_stub = ThermometerStub::new("bedroom temp sensor".to_string());
        term_stub.borrow_mut().current_temp_deg = 10.0;
        assert_eq!(term_stub.borrow().temperature_deg_celsius().unwrap().unwrap(), 10.0);
        term_stub.borrow_mut().online(false);
        assert!(term_stub.borrow().temperature_deg_celsius().is_err());
        term_stub.borrow_mut().online(true);
    }
}