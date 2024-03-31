pub mod smarthome {

    use std::{
        collections::HashSet,
        sync::atomic::{AtomicU64, Ordering},
    };

    // simple run-time only unique id generator
    fn gen_id() -> u64 {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub enum PowerState {
        #[default]
        OFF,
        ON,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum DeviceError {
        ERROR(u32),
    }

    // Describes device ID
    type DeviceID = u64;

    // temperature in fahrenheit
    type Temperature = f64;

    // trait for any IoT device
    pub trait IoTDevice {
        // unique ID of the device
        fn get_id(&self) -> DeviceID;
        // check if device is functioning properly
        fn test(&self) -> Result<(), DeviceError>;
        // human-readable description of the device
        fn status(&self) -> std::string::String;
    }

    // convert temperature from fahrenheit to celsius
    pub fn fahrenheit_to_celsius(t: Temperature) -> f64 {
        (t - 32.0) / 1.8
    }

    // convert temperature from celsius to fahrenheit
    pub fn celsius_to_fahrenheit(temp: f64) -> Temperature {
        (temp * 1.8) + 32.0
    }

    // trait for devices providing electric consumption meter
    pub trait ECMeter {
        fn get_consumption(&self) -> Result<f64, DeviceError>;
    }

    // trait for devices providing temperature info
    pub trait TempMeter {
        fn get_temperature(&self) -> Result<Temperature, DeviceError>;
    }

    // ActiveDevice describes device that can be turned on/off
    pub trait ActiveDevice {
        fn switch(&mut self, state: PowerState) -> Result<bool, DeviceError>;
        fn get_state(&self) -> Result<PowerState, DeviceError>;
    }

    // ACSocket
    #[derive(Default, Debug, Clone)]
    pub struct ACSocket {
        id: DeviceID,
        state: IoTBaseState,
    }

    // Temperature sensor
    #[derive(Default, Debug, Clone)]
    pub struct TempSensor {
        id: DeviceID,
        state: IoTBaseState,
    }

    // Describes common state of IoT devices (power on/off)
    #[derive(Default, Debug, Clone)]
    struct IoTBaseState {
        state: PowerState,
    }

    impl IoTBaseState {
        fn switch(&mut self, state: PowerState) -> Result<bool, DeviceError> {
            match (&self.state, &state) {
                (PowerState::ON, PowerState::ON) => Ok(false),
                (PowerState::OFF, PowerState::OFF) => Ok(false),
                (PowerState::OFF, PowerState::ON) => {
                    self.state = state;
                    Ok(true)
                }
                (PowerState::ON, PowerState::OFF) => {
                    self.state = state;
                    Ok(true)
                }
            }
        }

        fn get_state(&self) -> Result<PowerState, DeviceError> {
            Ok(self.state)
        }
    }

    // Temperature sensor
    impl TempSensor {
        pub fn new() -> TempSensor {
            TempSensor {
                id: gen_id(),
                state: IoTBaseState {
                    state: PowerState::OFF,
                },
            }
        }
    }

    impl ACSocket {
        pub fn new() -> ACSocket {
            ACSocket {
                id: gen_id(),
                state: IoTBaseState {
                    state: PowerState::OFF,
                },
            }
        }
    }

    impl IoTDevice for ACSocket {
        fn get_id(&self) -> DeviceID {
            self.id
        }

        fn test(&self) -> Result<(), DeviceError> {
            Ok(()) // todo: implement real diagnostic
        }

        fn status(&self) -> std::string::String {
            format!(
                "AC Socket {:?}, power state {:?}, consumption {:?}",
                self.id,
                self.state.state,
                self.get_consumption()
            )
        }
    }

    impl IoTDevice for TempSensor {
        fn get_id(&self) -> DeviceID {
            self.id
        }

        fn test(&self) -> Result<(), DeviceError> {
            Ok(()) // todo: implement real diagnostic
        }

        fn status(&self) -> std::string::String {
            format!(
                "Temp sensor {:?}, power state {:?}, temp {:?}",
                self.id,
                self.state.state,
                self.get_temperature()
            )
        }
    }

    impl ECMeter for ACSocket {
        fn get_consumption(&self) -> Result<f64, DeviceError> {
            // TODO: compute actual consumption
            Ok(1.2)
        }
    }

    impl TempMeter for TempSensor {
        fn get_temperature(&self) -> Result<Temperature, DeviceError> {
            // TODO: compute actual temperature
            Ok(celsius_to_fahrenheit(22.0))
        }
    }

    impl ActiveDevice for ACSocket {
        fn switch(&mut self, state: PowerState) -> Result<bool, DeviceError> {
            self.state.switch(state)
        }

        fn get_state(&self) -> Result<PowerState, DeviceError> {
            self.state.get_state()
        }
    }

    impl ActiveDevice for TempSensor {
        fn switch(&mut self, state: PowerState) -> Result<bool, DeviceError> {
            self.state.switch(state)
        }

        fn get_state(&self) -> Result<PowerState, DeviceError> {
            self.state.get_state()
        }
    }

    impl std::fmt::Display for ACSocket {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AC socket: id = {:?}, power state = {:?}, electric consumption = {:?}",
                self.id,
                self.state.state,
                self.get_consumption().unwrap()
            )
        }
    }

    impl std::fmt::Display for TempSensor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Temp sensor: id = {:?}, power state = {:?}, temp = {:?}",
                self.id,
                self.state.state,
                self.get_temperature().unwrap()
            )
        }
    }

    // Device ref represents short reference to IoT device in smarthome
    #[derive(Debug, Default, Clone)]
    pub struct DeviceRef(DeviceID, std::string::String);

    #[derive(Debug, Default, Clone)]
    pub struct Room {
        name: std::string::String,
        ac_sockets: std::vec::Vec<ACSocket>,
        t_sensors: std::vec::Vec<TempSensor>,
    }

    impl Room {
        pub fn builder() -> RoomBuilder {
            RoomBuilder::default()
        }

        pub fn get_name(&self) -> std::string::String {
            self.name.clone()
        }

        pub fn get_device_refs(&self) -> Vec<DeviceRef> {
            let mut result = vec![];
            self.ac_sockets
                .iter()
                .for_each(|s| result.push(DeviceRef(s.id, self.name.clone())));
            self.t_sensors
                .iter()
                .for_each(|t| result.push(DeviceRef(t.id, self.name.clone())));
            result
        }

        pub fn get_device_descriptions(&self) -> Vec<std::string::String> {
            let mut result = vec![];
            self.ac_sockets.iter().for_each(|ac| {
                result.push(format!("ac socket {:?}", ac.id));
            });
            self.t_sensors.iter().for_each(|ts| {
                result.push(format!("temp sensor {:?}", ts.id));
            });
            result
        }

        pub fn switch_state(&mut self, state: PowerState) -> &mut Self {
            self.ac_sockets.iter_mut().for_each(|s| {
                if s.switch(state).is_err() {
                    panic!("cannot change state")
                }
            });
            self.t_sensors.iter_mut().for_each(|s| {
                if s.switch(state).is_err() {
                    panic!("cannot change state")
                }
            });
            self
        }
    }

    impl std::fmt::Display for Room {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Room name = {:?}, temp sensor count = {:?}, ac socket count = {:?}",
                self.name,
                self.t_sensors.len(),
                self.ac_sockets.len()
            )
        }
    }

    #[derive(Debug, Default)]
    pub struct RoomBuilder {
        room: Room,
    }

    impl RoomBuilder {
        pub fn new(name: std::string::String) -> RoomBuilder {
            RoomBuilder {
                room: Room {
                    name: name.clone(),
                    ac_sockets: vec![],
                    t_sensors: vec![],
                },
            }
        }
        pub fn add_tsensor(mut self, sensor: TempSensor) -> Self {
            self.room.t_sensors.push(sensor);
            self
        }

        pub fn add_ac_socket(mut self, socket: ACSocket) -> Self {
            self.room.ac_sockets.push(socket);
            self
        }

        pub fn build(self) -> Room {
            self.room
        }
    }

    pub trait DeviceInfoProvider {
        fn is_included(&self, r: &DeviceRef) -> bool;

        fn get_device_refs(&self) -> std::vec::Vec<DeviceRef>;

        fn get_device_status(
            &self,
            house: &House,
            r: &DeviceRef,
        ) -> Result<std::string::String, std::string::String>;
    }

    #[derive(Debug, Clone)]
    pub struct OwningDeviceInfoProvider {
        room: std::string::String,
        socket: ACSocket,
        tsensor: TempSensor,
    }

    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn is_included(&self, r: &DeviceRef) -> bool {
            self.room == r.1 && (self.socket.id == r.0 || self.tsensor.id == r.0)
        }

        fn get_device_refs(&self) -> std::vec::Vec<DeviceRef> {
            vec![
                DeviceRef(self.socket.id, self.room.clone()),
                DeviceRef(self.tsensor.id, self.room.clone()),
            ]
        }

        fn get_device_status(
            &self,
            house: &House,
            r: &DeviceRef,
        ) -> Result<std::string::String, std::string::String> {
            if r.0 == self.socket.id || r.0 == self.tsensor.id {
                return house.get_device_status(r);
            }
            Err(format!(
                "ERROR: device {:?} is not included in this report",
                r.0
            ))
        }
    }

    impl OwningDeviceInfoProvider {
        pub fn new(
            room: std::string::String,
            s: ACSocket,
            ts: TempSensor,
        ) -> OwningDeviceInfoProvider {
            OwningDeviceInfoProvider {
                room,
                socket: s,
                tsensor: ts,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BorrowingDeviceInfoProvider<'a, 'b> {
        room: std::string::String,
        socket: &'a ACSocket,
        tsensor: &'b TempSensor,
    }

    impl<'a, 'b> BorrowingDeviceInfoProvider<'a, 'b> {
        pub fn new(room: std::string::String, s: &'a ACSocket, t: &'b TempSensor) -> Self {
            BorrowingDeviceInfoProvider {
                room,
                socket: s,
                tsensor: t,
            }
        }
    }

    impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
        fn is_included(&self, r: &DeviceRef) -> bool {
            self.room == r.1 && (self.socket.id == r.0 || self.tsensor.id == r.0)
        }

        fn get_device_refs(&self) -> std::vec::Vec<DeviceRef> {
            vec![
                DeviceRef(self.socket.id, self.room.clone()),
                DeviceRef(self.tsensor.id, self.room.clone()),
            ]
        }

        fn get_device_status(
            &self,
            house: &House,
            r: &DeviceRef,
        ) -> Result<std::string::String, std::string::String> {
            if r.0 == self.socket.id || r.0 == self.tsensor.id {
                return house.get_device_status(r);
            }
            Err(format!(
                "ERROR: device {:?} is not included in this report",
                r.0
            ))
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct House {
        name: std::string::String,
        rooms: std::vec::Vec<Room>,
    }

    impl House {
        pub fn get_name(&self) -> std::string::String {
            self.name.clone()
        }

        pub fn get_rooms(&self) -> &Vec<Room> {
            &self.rooms
        }

        pub fn builder() -> HouseBuilder {
            HouseBuilder::default()
        }

        pub fn turn_on(&mut self) -> &mut Self {
            self.rooms.iter_mut().for_each(|r| {
                r.switch_state(PowerState::ON);
            });
            self
        }

        pub fn turn_off(&mut self) -> &mut Self {
            self.rooms.iter_mut().for_each(|r| {
                r.switch_state(PowerState::OFF);
            });
            self
        }

        pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> std::string::String {
            let mut result = format!("House '{}'\n", self.name);
            provider
                .get_device_refs()
                .iter()
                .for_each(|r| match self.get_device_status(r) {
                    Ok(status) => {
                        result.push_str(
                            format!("Room: '{}', device status: '{}'", r.1, status).as_str(),
                        );
                        result.push('\n');
                    }
                    Err(err) => {
                        result.push_str(format!("ERROR: {}", err).as_str());
                        result.push('\n');
                    }
                });
            result
        }

        pub fn get_device_status(
            &self,
            r: &DeviceRef,
        ) -> Result<std::string::String, std::string::String> {
            for room in &self.rooms {
                for socket in &room.ac_sockets {
                    if socket.id == r.0 {
                        return Ok(socket.status());
                    }
                }
                for ts in &room.t_sensors {
                    if ts.id == r.0 {
                        return Ok(ts.status());
                    }
                }
            }
            Err(format!("ERROR: device {:?} is not found in the house", r.0))
        }
    }

    #[derive(Default)]
    pub struct HouseBuilder {
        house: House,
    }

    impl HouseBuilder {
        pub fn new(name: std::string::String) -> HouseBuilder {
            HouseBuilder {
                house: House {
                    name: name.clone(),
                    rooms: vec![],
                },
            }
        }

        pub fn add_room(mut self, r: Room) -> Self {
            self.house.rooms.push(r);
            self
        }

        pub fn build(self) -> Result<House, std::string::String> {
            let mut names = HashSet::new();
            let mut conflicts = Vec::default();
            self.house.rooms.iter().for_each(|r| {
                if !names.insert(r.name.clone()) {
                    conflicts.push(r.name.clone())
                }
            });
            if !conflicts.is_empty() {
                return Err(format!(
                    "Found rooms with conflicting names {:?}",
                    conflicts
                ));
            }
            Ok(self.house)
        }
    }
}

fn main() {
    let s1 = smarthome::ACSocket::new();
    let s2 = smarthome::ACSocket::new();
    let s3 = smarthome::ACSocket::new();
    let ts1 = smarthome::TempSensor::new();
    let ts2 = smarthome::TempSensor::new();
    let ts3 = smarthome::TempSensor::new();
    let ts4 = smarthome::TempSensor::new();

    let missing_socket = smarthome::ACSocket::new();
    let missing_tsensor = smarthome::TempSensor::new();

    let owning_reporter =
        smarthome::OwningDeviceInfoProvider::new(String::from("Room A"), s1.clone(), ts1.clone());

    let borrowing_reporter =
        smarthome::BorrowingDeviceInfoProvider::new(String::from("Room B"), &s3, &ts3);

    let invalid_owning_reporter = smarthome::OwningDeviceInfoProvider::new(
        String::from("Missing Room A"),
        missing_socket.clone(),
        missing_tsensor.clone(),
    );

    let invalid_borrowing_reporter = smarthome::BorrowingDeviceInfoProvider::new(
        String::from("Room A"),
        &missing_socket,
        &missing_tsensor,
    );

    let jack_home = smarthome::HouseBuilder::new(String::from("test"))
        .add_room(
            smarthome::RoomBuilder::new(String::from("Room A"))
                .add_ac_socket(s1)
                .add_ac_socket(s2)
                .add_tsensor(ts1)
                .build(),
        )
        .add_room(
            smarthome::RoomBuilder::new(String::from("Room B"))
                .add_ac_socket(s3.clone())
                .add_tsensor(ts2)
                .add_tsensor(ts3.clone())
                .add_tsensor(ts4)
                .build(),
        )
        .build();
    match jack_home {
        Ok(home) => {
            // Turn power on
            let mut owned_home = home.to_owned();
            owned_home.turn_on();

            // Room list
            println!("------------- HOUSE ---------------");
            owned_home.get_rooms().iter().for_each(|r| {
                println!("Room {:?}", r.get_name());
                r.get_device_descriptions()
                    .iter()
                    .for_each(|d| println!("    {:?}", d))
            });

            println!("------------- REPORTS ---------------");

            // Generate reports
            let report1 = owned_home.create_report(&owning_reporter);
            let report2 = owned_home.create_report(&borrowing_reporter);
            println!("Valid owning report:\n{}", report1);
            println!("Valid borrowing report:\n{}", report2);

            // Now reports for missing room/devices
            let ireport1 = owned_home.create_report(&invalid_owning_reporter);
            let ireport2 = owned_home.create_report(&invalid_borrowing_reporter);
            println!("Invalid owning report:\n{}", ireport1);
            println!("Invalid borrowing report:\n{}", ireport2);
        }
        Err(err) => println!("Cannot build house {}", err),
    }
}
