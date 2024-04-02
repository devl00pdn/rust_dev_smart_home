use crate::common::traits::Described;
use crate::common::traits::device::SmartDevice;

pub struct Room<'a> {
    name: String,
    devices: Vec<&'a dyn SmartDevice>,
}

impl Described for Room<'_> {
    fn description(&self) -> String {
        self.name.clone()
    }
}

impl<'a> Room<'a> {
    pub fn new(name: String) -> Room<'a> {
        Room { name, devices: vec![] }
    }

    pub fn add_device(&mut self, dev: &'a dyn SmartDevice) {
        self.devices.push(dev);
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for device in &self.devices {
            let desc = device.description();
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
        let mut room = Room::new("living room".to_string());
        let socket = SocketStub::new("base socket".to_string());
        let term = SocketStub::new("base thermometer".to_string());

        room.add_device(&socket as &dyn SmartDevice);
        room.add_device(&term as &dyn SmartDevice);

        let report = room.make_report();
        assert_eq!("base socket\nbase thermometer\n", report);
    }
}