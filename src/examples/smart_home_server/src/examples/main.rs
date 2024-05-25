use smart_home_lib::common::traits::device::Switchable;
use smart_home_lib::devices::stubs::socket_stub::SocketStub;
use smart_home_lib::devices::stubs::thermometer_stub::ThermometerStub;
use smart_home_lib::house::House;
use smart_home_lib::house::room::Room;

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
    home.add_room(livingroom.clone());
    home.add_room(kitchen.clone());
    println!("{}", home.make_report());

    socket.borrow_mut().online(false);
    match socket.borrow().current_state() {
        Ok(_) => {}
        Err(e) => { eprintln!("Error output: {}", e); }
    };
    socket.borrow_mut().online(true);

    kitchen.borrow_mut().remove_device("base thermometer".to_string()).expect("removing base thermometer");
    home.remove_room("kitchen".to_string()).expect("removing kitchen");
}
