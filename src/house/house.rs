use crate::common::traits::Described;
use crate::house::room::Room;

pub struct House<'a> {
    rooms: Vec<Room<'a>>,
}

impl House<'_> {
    pub fn new() -> House<'static> {
        House { rooms: vec![] }
    }

    pub fn make_report(&self) -> String {
        let mut report = String::new();
        for room in &self.rooms {
            report = format!("{}{}\n", report, room.description());
        }
        report
    }
}