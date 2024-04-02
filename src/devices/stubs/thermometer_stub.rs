use crate::common::traits::Described;
use crate::common::traits::device::{SmartDevice, Thermometer};
use crate::devices::thermometer::TemperatureSensorTrait;

#[derive(Debug)]
pub struct ThermometerStub {
    description: String,
    current_temp_deg: f32,
}

impl Described for ThermometerStub {
    fn description(&self) -> String {
        self.description.clone()
    }
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

impl SmartDevice for ThermometerStub {
    fn as_temperature_sensor(&self) -> Option<&dyn TemperatureSensorTrait> {
        Some(self)
    }
}

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