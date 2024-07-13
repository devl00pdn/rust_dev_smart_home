use std::cell::RefCell;
use std::collections::LinkedList;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::common::traits::Described;
use crate::common::types::SmartPointer;
use crate::devices::socket::SocketTrait;
use crate::devices::stubs::socket_stub::SocketStub;
use crate::devices::stubs::thermometer_stub::ThermometerStub;
use crate::devices::thermometer::TemperatureSensorTrait;

pub struct SpWrapper<T> {
    sp: SmartPointer<T>,
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

pub trait DeviceTypes {
    type Socket: SocketTrait;
    type Thermometer: TemperatureSensorTrait;
}

pub enum Device<T: DeviceTypes>
{
    Socket(SmartPointer<T::Socket>),
    Thermometer(SmartPointer<T::Thermometer>),
}

pub struct Room<T: DeviceTypes> {
    name: String,
    devices: LinkedList<SmartPointer<Device<T>>>,
}

impl<T: DeviceTypes> Room<T> {
    pub fn new(name: String) -> SmartPointer<Self> {
        Rc::new(RefCell::new(Self { name, devices: LinkedList::new() }))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn add_device(&mut self, device: SmartPointer<Device<T>>) {
        self.devices.push_back(device)
    }

    pub fn remove_device(&mut self, name: String) -> Result<(), crate::common::traits::device::Err> {
        let element_position = self.devices.iter_mut().position(|dev| {
            match dev.borrow_mut().deref_mut() {
                Device::Socket(d) => { d.borrow_mut().description() == name }
                Device::Thermometer(d) => { d.borrow_mut().description() == name }
            }
        });
        if let Some(remove_pos) = element_position {
            let swapped_elem = self.devices.pop_back().unwrap();
            if remove_pos < self.devices.len() {
                *self.devices.iter_mut().nth(remove_pos).unwrap() = swapped_elem;
            }
            return Ok(());
        }
        Err(crate::common::traits::device::Err { msg: "Device to remove not found".to_string() })
    }

    pub fn make_report(&mut self) -> String {
        let report = String::new();

        self.visit_mut(|device| {
            match device.borrow_mut().deref_mut() {
                Device::Socket(d) => { format!("{}{}\n", report, d.borrow_mut().description()); }
                Device::Thermometer(d) => { format!("{}{}\n", report, d.borrow_mut().description()); }
            }
        });
        report
    }

    pub fn visit_mut<F>(&mut self, mut visitor: F)
    where
        F: FnMut(&mut SmartPointer<Device<T>>),
    {
        for mut device in &mut self.devices {
            visitor(&mut device)
        }
    }

    pub fn visit<F>(&mut self, visitor: F)
    where
        F: Fn(&SmartPointer<Device<T>>),
    {
        for device in &self.devices {
            visitor(&device)
        }
    }
}

impl<T> From<SmartPointer<SocketStub>> for SmartPointer<Device<T>>
where
    T: DeviceTypes<Socket=SocketStub>,
{
    fn from(value: SmartPointer<SocketStub>) -> Self {
        Device::Socket(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::stubs::socket_stub::SocketStub;
    use crate::devices::stubs::thermometer_stub::ThermometerStub;

    use super::*;

    #[test]
    fn add_and_remove_devices() {
        let room = Room::new("living room".to_string());
        let socket = SocketStub::new("base socket".to_string());
        let term = ThermometerStub::new("base thermometer".to_string());

        room.borrow_mut().add_device(socket.clone().into());
        room.borrow_mut().add_device(term.clone().into());

        let report = room.borrow().make_report();
        assert_eq!("base socket\nbase thermometer\n", report);

        // check error on remove not existed device
        if let Err(err) = room.borrow_mut().remove_device("not_added_device_name".to_string()) {
            assert_eq!("Device to remove not found", err.msg);
        };
        // check report hasn't changed
        let report = room.borrow().make_report();
        assert_eq!("base socket\nbase thermometer\n", report);

        // remove device - base socket
        if let Err(err) = room.borrow_mut().remove_device("base thermometer".to_string()) {
            panic!("{}", err);
        };

        // check base socket has deleted
        let report = room.borrow().make_report();
        assert_eq!("base socket\n", report);

        // remove device - base thermometer
        if let Err(err) = room.borrow_mut().remove_device("base socket".to_string()) {
            panic!("{}", err);
        };
        // check base thermometer has deleted
        let report = room.borrow().make_report();
        assert_eq!("", report);

        // check error on remove empty device list
        if let Err(err) = room.borrow_mut().remove_device("base thermometer".to_string()) {
            assert_eq!("Device to remove not found", err.msg);
        };
    }
}



