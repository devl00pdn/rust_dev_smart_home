use crate::common_traits::device::{Described, Thermometer};

pub trait TemperatureSensorTrait: Thermometer + Described {}
