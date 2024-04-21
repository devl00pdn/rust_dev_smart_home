pub trait Described {
    fn description(&self) -> String {
        "none".to_string()
    }
}

pub mod device {
    pub trait SmartDevice: super::Described {}

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