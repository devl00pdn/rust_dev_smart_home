pub trait Described {
    fn description(&self) -> String {
        "none".to_string()
    }
}

pub mod device {
    use crate::devices::socket::SocketTrait;
    use crate::devices::thermometer::TemperatureSensorTrait;

    pub trait SmartDevice: super::Described {
        fn as_socket(&self) -> Option<&dyn SocketTrait> { None }
        fn as_temperature_sensor(&self) -> Option<&dyn TemperatureSensorTrait> { None }
    }

    pub trait Switchable {
        fn turn_on(&mut self) -> bool;
        fn turn_off(&mut self) -> bool;
        fn current_state(&self) -> bool;
    }

    pub trait PowerConsumptionMeter {
        fn power_consumption_wt(&self) -> f32;
    }

    pub trait Thermometer {
        fn temperature_deg_celsius(&self) -> f32;
    }
}