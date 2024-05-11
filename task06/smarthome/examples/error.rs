//! This example show usage of custom error type

use libsmarthome::factory::*;
use libsmarthome::home::*;
use libsmarthome::logical::*;
use libsmarthome::logical_device::Device;
use libsmarthome::room::*;

/// Show report generation for smart home
fn main() {
    // room label
    let rlabel1 = String::from("Room A");
    let rlabel2 = String::from("Room B");

    // Sockets (logical devices)
    let mut s1 = Device::new(String::from("socket"));
    let s2 = Device::new(String::from("socket"));
    let s3 = Device::new(String::from("socket"));
    let s4 = Device::new(String::from("socket"));

    // Temperature sensors (logical devices)
    let t1 = Device::new(String::from("tsensor"));
    let t2 = Device::new(String::from("tsensor"));
    let t3 = Device::new(String::from("tsensor"));

    // rooms
    let room1 = RoomBuilder::new()
        .set_label(rlabel1.clone())
        .add_device(&s1)
        .add_device(&s2)
        .add_device(&s3)
        .add_device(&t1)
        .add_device(&t2)
        .build();

    let room2 = RoomBuilder::new()
        .set_label(rlabel2.clone())
        .add_device(&s4)
        .add_device(&t3)
        .build();

    let mut my_home = HomeBuilder::new()
        .set_name(String::from("Jacks home"))
        .add_room(Box::new(room1))
        .add_room(Box::new(room2))
        .build();

    // Try to power on without binding to physical devices
    match my_home.switch_power(PowerState::ON) {
        Ok(_) => panic!("unexpected state, no physical devices bound"),
        Err(err) => println!("EXPECTED ERROR: {}", err), // this is expected behavior
    }

    // Try to bind logical device two times without unbind operation
    let factory = SimpleClassFactory {};
    let pd1 = factory.create_physical_device(&s1).unwrap();
    let pd2 = factory.create_physical_device(&s1).unwrap();
    match s1.bind(pd1) {
        Ok(_) => match s1.bind(pd2) {
            Ok(_) => panic!("unexpected double bind"),
            Err(v) => println!("EXPECTED ERROR: {}", v), // this is expected behavior
        },
        Err(_) => panic!("unexpected error"),
    }
}
