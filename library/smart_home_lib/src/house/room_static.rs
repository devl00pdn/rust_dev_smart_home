use std::cell::RefCell;
use std::collections::LinkedList;
use std::ops::{Deref, DerefMut};

use crate::common::traits::Described;
use crate::common::types::SmartPointer;
use crate::devices::socket::SocketTrait;
use crate::devices::stubs::socket_stub::SocketStub;
use crate::devices::stubs::thermometer_stub::ThermometerStub;
use crate::devices::thermometer::TemperatureSensorTrait;

pub struct SpWrapper<T> {
    sp: SmartPointer<T>,
}

impl<T> SpWrapper<T> {
    pub fn new(v: T) -> Self {
        Self {
            sp: SmartPointer::new(RefCell::new(v)),
        }
    }

    pub fn new_from_sp(sp: SmartPointer<T>) -> Self {
        Self { sp }
    }
}

impl<T> Deref for SpWrapper<T> {
    type Target = SmartPointer<T>;

    fn deref(&self) -> &Self::Target {
        &self.sp
    }
}

impl<T> DerefMut for SpWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sp
    }
}

impl<T> From<SpWrapper<SocketStub>> for Device<T>
where
    T: DeviceTypes<Socket = SocketStub>,
{
    fn from(value: SpWrapper<SocketStub>) -> Self {
        Device::Socket(value)
    }
}

impl<T> From<SpWrapper<ThermometerStub>> for Device<T>
where
    T: DeviceTypes<Thermometer = ThermometerStub>,
{
    fn from(value: SpWrapper<ThermometerStub>) -> Self {
        Device::Thermometer(value)
    }
}

impl<T> From<SmartPointer<T>> for SpWrapper<T> {
    fn from(value: SmartPointer<T>) -> Self {
        Self::new_from_sp(value)
    }
}

pub trait DeviceTypes {
    type Socket: SocketTrait;
    type Thermometer: TemperatureSensorTrait;
}

pub enum Device<T: DeviceTypes> {
    Socket(SpWrapper<T::Socket>),
    Thermometer(SpWrapper<T::Thermometer>),
}

pub struct Room<T: DeviceTypes> {
    name: String,
    devices: LinkedList<Device<T>>,
}

struct RoomStub;

impl DeviceTypes for RoomStub {
    type Socket = SocketStub;
    type Thermometer = ThermometerStub;
}

impl<T: DeviceTypes> Room<T> {
    pub fn new(name: String) -> SpWrapper<Self> {
        SpWrapper::new(Self {
            name,
            devices: LinkedList::new(),
        })
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn add_device(&mut self, device: Device<T>) {
        self.devices.push_back(device)
    }

    pub fn remove_device(
        &mut self,
        name: String,
    ) -> Result<(), crate::common::traits::device::Err> {
        let element_position = self.devices.iter_mut().position(|dev| match dev {
            Device::Socket(d) => d.borrow_mut().description() == name,
            Device::Thermometer(d) => d.borrow_mut().description() == name,
        });
        if let Some(remove_pos) = element_position {
            let swapped_elem = self.devices.pop_back().unwrap();
            if remove_pos < self.devices.len() {
                *self.devices.iter_mut().nth(remove_pos).unwrap() = swapped_elem;
            }
            return Ok(());
        }
        Err(crate::common::traits::device::Err {
            msg: "Device to remove not found".to_string(),
        })
    }

    pub fn make_report(&mut self) -> String {
        let mut report = String::new();

        self.visit_mut(|device| match device {
            Device::Socket(d) => {
                report = format!("{}{}\n", report, d.borrow_mut().description());
            }
            Device::Thermometer(d) => {
                report = format!("{}{}\n", report, d.borrow_mut().description());
            }
        });
        report
    }

    pub fn visit_mut<F>(&mut self, mut visitor: F)
    where
        F: FnMut(&mut Device<T>),
    {
        for device in &mut self.devices {
            visitor(device)
        }
    }

    pub fn visit<F>(&mut self, visitor: F)
    where
        F: Fn(&Device<T>),
    {
        for device in &self.devices {
            visitor(device)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::stubs::socket_stub::SocketStub;
    use crate::devices::stubs::thermometer_stub::ThermometerStub;

    use super::*;

    #[test]
    fn add_and_remove_devices() {
        let mut room = Room::<RoomStub>::new("living room".to_string());
        let socket: SpWrapper<SocketStub> = SocketStub::new("base socket".to_string()).into();
        let term: SpWrapper<ThermometerStub> =
            ThermometerStub::new("base thermometer".to_string()).into();

        room.borrow_mut().add_device(socket.into());
        room.borrow_mut().add_device(term.into());
        let report = room.deref_mut().borrow_mut().make_report();

        assert_eq!("base socket\nbase thermometer\n", report);

        // check error on remove not existed device
        if let Err(err) = room
            .deref_mut()
            .borrow_mut()
            .remove_device("not_added_device_name".to_string())
        {
            assert_eq!("Device to remove not found", err.msg);
        };
        // check report hasn't changed
        let report = room.deref_mut().borrow_mut().make_report();
        assert_eq!("base socket\nbase thermometer\n", report);

        // remove device - base socket
        if let Err(err) = room
            .deref_mut()
            .borrow_mut()
            .remove_device("base thermometer".to_string())
        {
            panic!("{}", err);
        };

        // check base socket has deleted
        let report = room.deref_mut().borrow_mut().make_report();
        assert_eq!("base socket\n", report);

        // remove device - base thermometer
        if let Err(err) = room
            .deref_mut()
            .borrow_mut()
            .remove_device("base socket".to_string())
        {
            panic!("{}", err);
        };
        // check base thermometer has deleted
        let report = room.deref_mut().borrow_mut().make_report();
        assert_eq!("", report);

        // check error on remove empty device list
        if let Err(err) = room
            .deref_mut()
            .borrow_mut()
            .remove_device("base thermometer".to_string())
        {
            assert_eq!("Device to remove not found", err.msg);
        };
    }
}
