//! This example show usage of add/remove functionality

use libsmarthome::home;
use libsmarthome::logical_device::Device;
use libsmarthome::room::*;

/// Show how to add/remove devices and rooms
fn main() {
    let mut smart_home = home::HomeBuilder::new()
        .set_name("Jack's home".to_string())
        .build();
    let r1 = smart_home.add_room(Box::new(
        RoomBuilder::new().set_label("Room 1".to_string()).build(),
    ));
    let r2 = smart_home.add_room(Box::new(
        RoomBuilder::new().set_label("Room 2".to_string()).build(),
    ));

    let device_id = smart_home
        .get_room_mut(&r1)
        .unwrap()
        .add_device(Device::new("socket".to_string()));
    smart_home
        .get_room_mut(&r1)
        .unwrap()
        .add_device(Device::new("socket".to_string()));
    smart_home
        .get_room_mut(&r1)
        .unwrap()
        .add_device(Device::new("tsensor".to_string()));

    smart_home
        .get_room_mut(&r2)
        .unwrap()
        .add_device(Device::new("socket".to_string()));
    smart_home
        .get_room_mut(&r2)
        .unwrap()
        .add_device(Device::new("tsensor".to_string()));

    println!("------------- INITIAL DEVICE LAYOUT----------------------");
    let mut devices = smart_home.get_devices();
    devices.sort();
    devices.iter().for_each(|r| println!("{}", r));
    println!("--------------------------------------------------------");

    // adding new sensor and room
    smart_home
        .get_room_mut(&r2)
        .unwrap()
        .add_device(Device::new("tsensor".to_string()));

    let r3 = smart_home.add_room(Box::new(
        RoomBuilder::new().set_label("Room 3".to_string()).build(),
    ));

    smart_home
        .get_room_mut(&r3)
        .unwrap()
        .add_device(Device::new("socket".to_string()));

    println!("------------- DEVICE LAYOUT AFTER ADDITIONS  ------------");
    let mut devices = smart_home.get_devices();
    devices.sort();
    devices.iter().for_each(|r| println!("{}", r));
    println!("--------------------------------------------------------");

    // Now remove devices and rooms
    smart_home.remove_room_by_label(&"Room 2".to_string());
    smart_home
        .get_room_mut(&r1)
        .unwrap()
        .remove_device(&device_id);

    println!("------------- DEVICE LAYOUT AFTER REMOVALS  ------------");
    let mut devices = smart_home.get_devices();
    devices.sort();
    devices.iter().for_each(|r| println!("{}", r));
    println!("--------------------------------------------------------");
}
