use smart_home_derive::Described;

use crate::common::traits::Described;
use crate::common::traits::device::{SmartDevice, Thermometer};
use crate::devices::thermometer::TemperatureSensorTrait;

#[derive(Debug, Described)]
pub struct ThermometerStub {
    description: String,
    current_temp_deg: f32,
}

impl Thermometer for ThermometerStub {
    fn temperature_deg_celsius(&self) -> f32 {
        self.current_temp_deg
    }
}

impl ThermometerStub {
    pub fn new(description: String) -> ThermometerStub {
        ThermometerStub { description, current_temp_deg: 0.0 }
    }
}

impl TemperatureSensorTrait for ThermometerStub {}

impl SmartDevice for ThermometerStub {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods() {
        let mut term_stub = ThermometerStub::new("bedroom temp sensor".to_string());
        term_stub.current_temp_deg = 10.0;
        assert_eq!(term_stub.temperature_deg_celsius(), 10.0);
    }
}