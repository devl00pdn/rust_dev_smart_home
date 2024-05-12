use rust_dev_smart_home::common::traits::device::Switchable;
use rust_dev_smart_home::devices::stubs::socket_stub::SocketStub;
use rust_dev_smart_home::devices::stubs::thermometer_stub::ThermometerStub;
use rust_dev_smart_home::house::House;
use rust_dev_smart_home::house::room::Room;

fn main() {
    let livingroom = Room::new("living room".to_string());
    let kitchen = Room::new("kitchen".to_string());
    let socket = SocketStub::new("base socket".to_string());
    let term = ThermometerStub::new("base thermometer".to_string());
    livingroom.borrow_mut().add_device(socket.clone());
    livingroom.borrow_mut().add_device(term.clone());
    kitchen.borrow_mut().add_device(socket.clone());
    kitchen.borrow_mut().add_device(term.clone());

    let mut home = House::new();
    home.add_room(livingroom);
    home.add_room(kitchen);
    println!("{}", home.make_report());

    socket.borrow_mut().online(false);
    match socket.borrow().current_state() {
        Ok(_) => {}
        Err(e) => { eprintln!("Error output: {}", e); }
    };
    socket.borrow_mut().online(true);
}
