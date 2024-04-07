use crate::common::traits::Described;
use crate::house::room::Room;

pub mod room;

pub struct House<'a> {
    rooms: Vec<&'a Room<'a>>,
}

impl<'a> Default for House<'a> {
    fn default() -> House<'a> {
        Self::new()
    }
}


impl<'a> House<'a> {
    pub fn new() -> House<'a> {
        House { rooms: vec![] }
    }

    pub fn add_room(&mut self, room: &'a Room) {
        self.rooms.push(room);
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            report = format!("{}{}:\n", report, room.description());
            report = format!("{}{}\n", report, room.make_report());
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use crate::common::traits::device::SmartDevice;
    use crate::devices::stubs::socket_stub::SocketStub;

    use super::*;

    #[test]
    fn add_rooms() {
        let mut livingroom = Room::new("living room".to_string());
        let mut kitchen = Room::new("kitchen".to_string());

        let socket = SocketStub::new("base socket".to_string());
        let term = SocketStub::new("base thermometer".to_string());

        livingroom.add_device(&socket as &dyn SmartDevice);
        livingroom.add_device(&term as &dyn SmartDevice);
        kitchen.add_device(&socket as &dyn SmartDevice);
        kitchen.add_device(&term as &dyn SmartDevice);
        let mut home = House::new();
        home.add_room(&livingroom);
        home.add_room(&kitchen);
        assert_eq!("living room:\nbase socket\nbase thermometer\n\nkitchen:\nbase socket\nbase thermometer\n\n", home.make_report());
    }
}