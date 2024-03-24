use devices::stubs::socket_stub::SocketStub;
use devices::stubs::thermometer_stub::ThermometerStub;

use crate::common_traits::device::SmartDevice;

mod common_traits;
mod devices;

fn main() {
    let smart_devices: Vec<Box<dyn SmartDevice>> = vec![Box::new(SocketStub::new("Kitchen socket".to_string())),
                                                        Box::new(ThermometerStub::new("Bedroom thermometer".to_string()))];

    println!("Hello, smart home! I have some devices: ");
    for dev in smart_devices {
        println!("{}", dev.description());
    }
}
