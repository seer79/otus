use smarthome::ACSocket;

pub mod smarthome {

    use std::time::{SystemTime, UNIX_EPOCH};

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

    // ECMeter describes device providing electric consumption meter
    pub trait ECMeter {
        fn get_consumption(&self) -> f64;
    }

    // Common trait for any IoT device (sensor, power supply)
    pub trait IoTDevice {
        // unique ID of the device
        fn id(&self) -> u64;
        // check if IoT device is functioning properly
        fn test(&self) -> Result<(), DeviceError>;
    }

    // ActiveDevice describes device that can be turned on/off
    pub trait ActiveDevice {
        fn switch(&mut self, state: PowerState) -> Result<bool, DeviceError>;
        fn get_state(&self) -> Result<PowerState, DeviceError>;
    }

    // Describes common state of IoT devices (power on/off)
    #[derive(Default)]
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

    // AC Power socket
    pub struct ACSocket {
        id: u64,
        state: IoTBaseState,
    }

    // Temperature sensor
    pub struct TempSensor {
        id: u64,
        state: IoTBaseState,
    }

    impl TempSensor {
        pub fn new() -> TempSensor {
            let id = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("invalid time")
                .as_millis();
            TempSensor {
                id: id as u64,
                state: IoTBaseState {
                    state: PowerState::OFF,
                },
            }
        }
    }

    impl IoTDevice for ACSocket {
        fn id(&self) -> u64 {
            self.id
        }

        fn test(&self) -> Result<(), DeviceError> {
            Ok(()) // todo: implement real diagnostic
        }
    }

    impl ECMeter for ACSocket {
        fn get_consumption(&self) -> f64 {
            todo!("compute insance consumption")
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

    impl std::fmt::Display for ACSocket {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "id = {:?}, power state = {:?}",
                self.id, self.state.state
            )
        }
    }

    impl std::fmt::Display for TempSensor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "id = {:?}, power state = {:?}",
                self.id, self.state.state
            )
        }
    }
}

fn main() {
    let socket = smarthome::TempSensor::new();
    println!("{}", socket);
}
