use crate::common::traits::Described;
use crate::common::traits::device::Thermometer;

pub trait TemperatureSensorTrait: Thermometer + Described {}
