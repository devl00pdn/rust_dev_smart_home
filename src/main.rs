use devices::stubs::socket_stub::SocketStub;

use crate::common::traits::device::SmartDevice;
use crate::devices::stubs::thermometer_stub::ThermometerStub;
use crate::house::house::House;
use crate::house::room::Room;

mod devices;
mod house;
mod common;

fn main() {
    let mut livingroom = Room::new("living room".to_string());
    let mut kitchen = Room::new("kitchen".to_string());

    let socket = SocketStub::new("base socket".to_string());
    let term = ThermometerStub::new("base thermometer".to_string());

    livingroom.add_device(&socket as &dyn SmartDevice);
    livingroom.add_device(&term as &dyn SmartDevice);
    kitchen.add_device(&socket as &dyn SmartDevice);
    kitchen.add_device(&term as &dyn SmartDevice);

    let mut home = House::new();
    home.add_room(&livingroom);
    home.add_room(&kitchen);
    println!("{}", home.make_report());
}
