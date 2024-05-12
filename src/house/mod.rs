use std::cell::RefCell;
use std::rc::Rc;

use crate::common::traits::Described;
use crate::house::room::Room;

pub mod room;

pub struct House {
    rooms: Vec<Rc<RefCell<Room>>>,
}

impl Default for House {
    fn default() -> House {
        Self::new()
    }
}


impl House {
    pub fn new() -> House {
        House { rooms: vec![] }
    }

    pub fn add_room(&mut self, room: Rc<RefCell<Room>>) {
        self.rooms.push(room);
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            report = format!("{}{}:\n", report, room.borrow().description());
            report = format!("{}{}\n", report, room.borrow().make_report());
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::stubs::socket_stub::SocketStub;

    use super::*;

    #[test]
    fn add_rooms() {
        let livingroom = Room::new("living room".to_string());
        let kitchen = Room::new("kitchen".to_string());

        let socket = SocketStub::new("base socket".to_string());
        let term = SocketStub::new("base thermometer".to_string());

        livingroom.borrow_mut().add_device(socket.clone());
        livingroom.borrow_mut().add_device(term.clone());
        kitchen.borrow_mut().add_device(socket.clone());
        kitchen.borrow_mut().add_device(term.clone());
        let mut home = House::new();
        home.add_room(livingroom.clone());
        home.add_room(kitchen.clone());
        assert_eq!("living room:\nbase socket\nbase thermometer\n\nkitchen:\nbase socket\nbase thermometer\n\n", home.make_report());
    }
}