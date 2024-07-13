use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

use crate::common::traits::Described;
use crate::house::room::Room;

pub mod room;
pub mod room_static;

pub struct House {
    rooms: LinkedList<Rc<RefCell<Room>>>,
}

impl Default for House {
    fn default() -> House {
        Self::new()
    }
}


impl House {
    pub fn new() -> House {
        House { rooms: LinkedList::new() }
    }

    pub fn add_room(&mut self, room: Rc<RefCell<Room>>) {
        self.rooms.push_back(room);
    }

    pub fn remove_room(&mut self, name: String) -> Result<(), &str> {
        let room_position = self.rooms.iter().position(|room| room.borrow_mut().description() == name);
        if let Some(rm_index) = room_position {
            let swapped_elem = self.rooms.pop_back().unwrap();
            if rm_index < self.rooms.len() {
                *self.rooms.iter_mut().nth(rm_index).unwrap() = swapped_elem.clone();
            }
            return Ok(());
        }
        Err("Room to remove not found")
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            report = format!("{}{}:\n", report, room.borrow_mut().description());
            report = format!("{}{}\n", report, room.borrow_mut().make_report());
        }
        report
    }
    pub fn rooms_report(&self) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            report = format!("{}{}\n", report, room.borrow().name());
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::stubs::socket_stub::SocketStub;

    use super::*;

    #[test]
    fn add_and_remove_rooms() {
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
        assert_eq!("living room\nkitchen\n", home.rooms_report());
        if let Err(err) = home.remove_room("not_added_room_name".to_string()) {
            assert_eq!(err, "Room to remove not found");
        }
        if home.remove_room("kitchen".to_string()).is_err() {
            panic!("not expected result");
        }
        assert_eq!("living room\n", home.rooms_report());
        if home.remove_room("living room".to_string()).is_err() {
            panic!("not expected result");
        }
        assert_eq!("", home.rooms_report());
        if let Err(err) = home.remove_room("kitchen".to_string()) {
            assert_eq!(err, "Room to remove not found");
        }
    }
}