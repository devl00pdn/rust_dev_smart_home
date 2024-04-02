use crate::common::traits::Described;
use crate::common::traits::device::{PowerConsumptionMeter, Switchable};

pub trait SocketTrait: PowerConsumptionMeter + Switchable + Described {}

