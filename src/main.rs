use rust_dev_smart_home::common::traits::device::{SmartDevice, Switchable};
use rust_dev_smart_home::devices::stubs::socket_stub::SocketStub;
use rust_dev_smart_home::devices::stubs::thermometer_stub::ThermometerStub;
use rust_dev_smart_home::house::House;
use rust_dev_smart_home::house::room::Room;

fn main() {
    let mut livingroom = Room::new("living room".to_string());
    let mut kitchen = Room::new("kitchen".to_string());
    let mut socket = SocketStub::new("base socket".to_string());
    let term = ThermometerStub::new("base thermometer".to_string());
    livingroom.add_device(&socket as &dyn SmartDevice);
    livingroom.add_device(&term as &dyn SmartDevice);
    kitchen.add_device(&socket as &dyn SmartDevice);
    kitchen.add_device(&term as &dyn SmartDevice);

    let mut home = House::new();
    home.add_room(&livingroom);
    home.add_room(&kitchen);
    println!("{}", home.make_report());
  
    socket.online(false);
    match socket.current_state() {
        Ok(_) => {}
        Err(e) => { eprintln!("Error output: {}", e); }
    };
    socket.online(true);
}
