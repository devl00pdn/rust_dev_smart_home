use crate::common_traits::device::{Described, PowerConsumptionMeter, Switchable};

pub trait SocketTrait: PowerConsumptionMeter + Switchable + Described {}

