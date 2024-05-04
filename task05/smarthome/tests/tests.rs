//! Integration tests

use libsmarthome::factory::*;
use libsmarthome::home::*;
use libsmarthome::logical::*;
use libsmarthome::logical_device::Device;
use libsmarthome::report::SimpleReporter;
use libsmarthome::room::*;

/// Integration test for valid devices in the home
#[test]
fn itest_valid_devices() {
    // factory for generating physical devices for logical one
    let factory = SimpleClassFactory {};
    let binder = Binder::new(factory);

    // room label
    let rlabel = String::from("Room A");

    // Sockets (logical devices)
    let s1 = Device::new(String::from("socket"));
    let s2 = Device::new(String::from("socket"));
    let s3 = Device::new(String::from("socket"));

    // Temperature sensors (logical devices)
    let t1 = Device::new(String::from("tsensor"));
    let t2 = Device::new(String::from("tsensor"));

    // room
    let room1 = RoomBuilder::new()
        .set_label(rlabel.clone())
        .add_device(&s1)
        .add_device(&s2)
        .add_device(&s3)
        .add_device(&t1)
        .add_device(&t2)
        .build();
    let rid1 = room1.get_id();
    let mut my_home = HomeBuilder::new()
        .set_name(String::from("Jacks home"))
        .add_room(Box::new(room1))
        .build();

    // test binding physical devices
    assert!(my_home.bind_physical_devices(&binder).is_ok());

    // turn power on
    assert!(my_home.switch_power(PowerState::ON).is_ok());

    let mut reporter = SimpleReporter::default();
    reporter.add_device(&rid1, &s1.get_id());
    reporter.add_device(&rid1, &s2.get_id());
    reporter.add_device(&rid1, &s3.get_id());
    reporter.add_device(&rid1, &t1.get_id());
    reporter.add_device(&rid1, &t2.get_id());

    let report = my_home.create_report(&reporter);
    println!("{}", report);

    // check that report does not include errors
    assert!(!report.contains("ERROR"));
}

#[test]
fn itest_missing_devices() {
    // factory for generating physical devices for logical one
    let factory = SimpleClassFactory {};
    let binder = Binder::new(factory);

    // room label
    let rlabel = String::from("Room A");

    // Sockets (logical devices)
    let s1 = Device::new(String::from("socket"));
    let s2 = Device::new(String::from("socket"));
    let s3 = Device::new(String::from("socket"));

    // Temperature sensors (logical devices)
    let t1 = Device::new(String::from("tsensor"));
    let t2 = Device::new(String::from("tsensor"));

    // room
    let room1 = RoomBuilder::new()
        .set_label(rlabel.clone())
        .add_device(&s1)
        .add_device(&t1)
        .build();
    let rid1 = room1.get_id();
    let mut my_home = HomeBuilder::new()
        .set_name(String::from("Jacks home"))
        .add_room(Box::new(room1))
        .build();

    // test binding physical devices
    assert!(my_home.bind_physical_devices(&binder).is_ok());

    // turn power on
    assert!(my_home.switch_power(PowerState::ON).is_ok());

    let mut reporter = SimpleReporter::default();
    reporter.add_device(&rid1, &s1.get_id());
    reporter.add_device(&rid1, &s2.get_id());
    reporter.add_device(&rid1, &s3.get_id());
    reporter.add_device(&rid1, &t1.get_id());
    reporter.add_device(&rid1, &t2.get_id());

    let report = my_home.create_report(&reporter);
    println!("{}", report);

    // Check that report includes errors
    assert!(report.contains("ERROR"));
}
