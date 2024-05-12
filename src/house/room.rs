use std::cell::RefCell;
use std::rc::Rc;

use crate::common::traits::Described;
use crate::common::traits::device::SmartDevice;
use crate::common::types::SmartPointer;

pub struct Room {
    name: String,
    devices: Vec<SmartPointer<dyn SmartDevice>>,
}

impl Described for Room {
    fn description(&self) -> String {
        self.name.clone()
    }
}

impl Room {
    pub fn new(name: String) -> SmartPointer<Room> {
        Rc::new(RefCell::new(Room { name, devices: vec![] }))
    }

    pub fn add_device(&mut self, dev: SmartPointer<dyn SmartDevice>) {
        self.devices.push(dev);
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for device in &self.devices {
            let desc = device.borrow().description();
            report = format!("{}{}\n", report, desc);
        }
        report
    }
}


#[cfg(test)]
mod tests {
    use crate::devices::stubs::socket_stub::SocketStub;

    use super::*;

    #[test]
    fn add_devices() {
        let room = Room::new("living room".to_string());
        let socket = SocketStub::new("base socket".to_string());
        let term = SocketStub::new("base thermometer".to_string());

        room.borrow_mut().add_device(socket.clone());
        room.borrow_mut().add_device(term.clone());

        let report = room.borrow().make_report();
        assert_eq!("base socket\nbase thermometer\n", report);
    }
}