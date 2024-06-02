use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use crate::common::traits::Described;
use crate::common::traits::device::{Err, SmartDevice};
use crate::common::types::SmartPointer;

pub struct Room {
    name: String,
    devices: LinkedList<SmartPointer<dyn SmartDevice>>,
}

impl Described for Room {
    fn description(&mut self) -> String {
        self.name.clone()
    }
}

impl Room {
    pub fn new(name: String) -> SmartPointer<Room> {
        Rc::new(RefCell::new(Room { name, devices: LinkedList::new() }))
    }

    pub fn add_device(&mut self, dev: SmartPointer<dyn SmartDevice>) {
        self.devices.push_back(dev);
    }

    pub fn remove_device(&mut self, name: String) -> Result<(), Err> {
        let element_position = self.devices.iter().position(|dev| dev.borrow_mut().description() == name);
        if let Some(remove_pos) = element_position {
            let swapped_elem = self.devices.pop_back().unwrap();
            if remove_pos < self.devices.len() {
                *self.devices.iter_mut().nth(remove_pos).unwrap() = swapped_elem.clone();
            }
            return Ok(());
        }
        Err(Err { msg: "Device to remove not found".to_string() })
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for device in &self.devices {
            let desc = device.borrow_mut().description();
            report = format!("{}{}\n", report, desc);
        }
        report
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::devices::stubs::socket_stub::SocketStub;

    use super::*;

    #[test]
    fn add_and_remove_devices() {
        let room = Room::new("living room".to_string());
        let socket = SocketStub::new("base socket".to_string());
        let term = SocketStub::new("base thermometer".to_string());

        room.borrow_mut().add_device(socket.clone());
        room.borrow_mut().add_device(term.clone());

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