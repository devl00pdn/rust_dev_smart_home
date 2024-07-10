use crate::common::traits::Described as DescribedStd;
use crate::common::traits::device::{PowerConsumptionMeter as PowerConsumptionMeterStd, Switchable as SwitchableStd};
use crate::common::traits_async::Described as DescribedAsync;
use crate::common::traits_async::device::{PowerConsumptionMeter as PowerConsumptionMeterAsync, Switchable as SwitchableAsync};

pub trait SocketTrait: PowerConsumptionMeterStd + SwitchableStd + DescribedStd {}
pub trait SocketTraitAsync: PowerConsumptionMeterAsync + SwitchableAsync + DescribedAsync {}

