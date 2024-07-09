use crate::common::traits::Described as DescribedStd;
use crate::common::traits::device::Thermometer as ThermometerStd;
use crate::common::traits_async::Described as DescribedAsync;
use crate::common::traits_async::device::Thermometer as ThermometerAsync;

pub trait TemperatureSensorTrait: ThermometerStd + DescribedStd {}
pub trait TemperatureSensorTraitAsync: ThermometerAsync + DescribedAsync {}
