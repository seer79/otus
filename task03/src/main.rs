pub mod smarthome {

    use std::sync::atomic::{AtomicU64, Ordering};

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
    #[derive(Default)]
    pub struct ACSocket {
        id: DeviceID,
        state: IoTBaseState,
    }

    // Temperature sensor
    #[derive(Default)]
    pub struct TempSensor {
        id: DeviceID,
        state: IoTBaseState,
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
    }

    impl IoTDevice for TempSensor {
        fn get_id(&self) -> DeviceID {
            self.id
        }

        fn test(&self) -> Result<(), DeviceError> {
            Ok(()) // todo: implement real diagnostic
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
}

fn main() {
    let socket = smarthome::TempSensor::new();
    println!("{}", socket);
}

#[cfg(test)]
mod tests {
    use crate::smarthome::{self, ActiveDevice, ECMeter, IoTDevice, TempMeter};

    #[test]
    fn test_creation() {
        let tsensor = smarthome::TempSensor::new();
        let socket = smarthome::ACSocket::new();
        assert!(socket.get_id() != tsensor.get_id());
    }

    #[test]
    fn test_temp() {
        let tsensor = smarthome::TempSensor::new();
        assert!(tsensor.get_temperature().unwrap() > 0.0);
    }

    #[test]
    fn test_consumption() {
        let socket = smarthome::ACSocket::new();
        assert!(socket.get_consumption().unwrap() > 0.0);
    }

    #[test]
    fn test_to_string() {
        assert!(smarthome::ACSocket::new()
            .to_string()
            .contains("electric consumption = 1.2"));
        assert!(smarthome::TempSensor::new()
            .to_string()
            .contains("temp = 71.6"));
    }

    #[test]
    fn test_switch() {
        let mut socket = smarthome::ACSocket::new();
        match socket.get_state() {
            Ok(smarthome::PowerState::OFF) => (),
            Ok(_) => panic!("invalid state"),
            Err(_) => panic!("invalid state"),
        }
        match socket.switch(smarthome::PowerState::ON) {
            Ok(_) => (),
            Err(_) => panic!("cannot switch state"),
        }
        match socket.get_state() {
            Ok(smarthome::PowerState::ON) => (),
            Ok(_) => panic!("invalid state"),
            Err(_) => panic!("invalid state"),
        }
        match socket.switch(smarthome::PowerState::OFF) {
            Ok(_) => (),
            Err(_) => panic!("cannot switch state"),
        }
        match socket.get_state() {
            Ok(smarthome::PowerState::OFF) => (),
            Ok(_) => panic!("invalid state"),
            Err(_) => panic!("invalid state"),
        }
    }
}
