use libsmarthome::factory::*;
use libsmarthome::home::*;
use libsmarthome::logical::*;
use libsmarthome::logical_device::Device;
use libsmarthome::report::SimpleReporter;
use libsmarthome::room::*;

/// Show report generation for smart home
fn main() {
    // factory for generating physical devices for logical one
    let factory = SimpleClassFactory {};
    let binder = Binder::new(factory);

    // room label
    let rlabel1 = String::from("Room A");
    let rlabel2 = String::from("Room B");

    // Sockets (logical devices)
    let s1 = Device::new(String::from("socket"));
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

    let rid1 = room1.get_id();
    let rid2 = room2.get_id();
    let mut my_home = HomeBuilder::new()
        .set_name(String::from("Jacks home"))
        .add_room(Box::new(room1))
        .add_room(Box::new(room2))
        .build();

    // bind  physical devices
    match my_home.bind_physical_devices(&binder) {
        Ok(_) => println!("bound logical devices to physical"),
        Err(_) => panic!("Cannot bind to physical devices"),
    }

    // turn power on
    match my_home.switch_power(PowerState::ON) {
        Ok(_) => println!("Power is ON"),
        Err(_) => panic!("Cannot turn power ON"),
    }

    // create reporter
    let mut reporter = SimpleReporter::default();
    reporter.add_device(&rid1, &s1.get_id());
    reporter.add_device(&rid1, &s2.get_id());
    reporter.add_device(&rid1, &s3.get_id());
    reporter.add_device(&rid1, &t1.get_id());
    reporter.add_device(&rid1, &t2.get_id());
    reporter.add_device(&rid2, &s4.get_id());
    reporter.add_device(&rid2, &t3.get_id());

    let report = my_home.create_report(&reporter);
    println!("{}", report);
}
