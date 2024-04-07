use std::f32::NAN;

use crate::commands::CMD_GET_POWER_CONSUMPTION;
use crate::commands::CMD_GET_TEMPERATURE;
use crate::commands::CMD_SELF_TEST;
use crate::commands::CMD_STATUS;
use crate::logical;
use crate::logical::*;
use crate::physical::*;
use rand::Rng;

/// TSensor describes temperature sensor
pub struct TSensor {
    serial: String,
    manufactor: String,
    state: logical::PowerState,
}

// convert temperature from fahrenheit to celsius
pub fn fahrenheit_to_celsius(t: f32) -> f32 {
    (t - 32.0) / 1.8
}

// convert temperature from celsius to fahrenheit
pub fn celsius_to_fahrenheit(temp: f32) -> f32 {
    (temp * 1.8) + 32.0
}

impl TSensor {
    pub fn new(serial: String, manufactor: String) -> Self {
        TSensor {
            serial,
            manufactor,
            state: PowerState::OFF,
        }
    }

    /// Temp sensor also consumes power
    fn get_consumption(&self) -> f32 {
        match &self.state {
            PowerState::OFF => 0.0,
            PowerState::ON => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0.1..1.0)
            }
        }
    }

    /// Returns current temperature as celsius
    fn get_temperature(&self) -> f32 {
        match &self.state {
            PowerState::OFF => NAN,
            PowerState::ON => {
                let mut rng = rand::thread_rng();
                rng.gen_range(10.0..32.0)
            }
        }
    }

    fn get_status(&self) -> String {
        format!(
            "Temperature sensor: serial = {}, manufactor = {}, power state = {:?}, consumption = {}, temperature = {}C",
            self.serial,
            self.manufactor,
            self.state,
            self.get_consumption(),
            self.get_temperature(),
        )
    }
}

impl PhysicalDevice for TSensor {
    fn get_serial(&self) -> String {
        self.serial.clone()
    }

    fn get_manufactor(&self) -> String {
        self.manufactor.clone()
    }

    fn get_power_state(&self) -> Result<PowerState, ErrorCode> {
        Ok(self.state)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<PowerState, ErrorCode> {
        self.state = state;
        Ok(self.state)
    }

    fn get_supported_commands(&self) -> Result<Vec<DeviceCommand>, ErrorCode> {
        Ok(vec![
            CMD_STATUS,
            CMD_GET_POWER_CONSUMPTION,
            CMD_SELF_TEST,
            CMD_GET_TEMPERATURE,
        ])
    }

    fn execute_cmd_mut(
        &mut self,
        cmd: DeviceCommand,
        _args: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, ErrorCode> {
        match cmd {
            CMD_GET_POWER_CONSUMPTION => Ok(Some(CommandResult::Float32(self.get_consumption()))),
            CMD_SELF_TEST => Ok(Some(CommandResult::Str(String::from("PASSED")))),
            CMD_STATUS => Ok(Some(CommandResult::Str(self.get_status()))),
            CMD_GET_TEMPERATURE => Ok(Some(CommandResult::Float32(self.get_temperature()))),
            _ => return Err(ErrorCode::UnsupportedCommand),
        }
    }

    fn execute_cmd(
        &self,
        cmd: DeviceCommand,
        _args: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, ErrorCode> {
        match cmd {
            CMD_GET_POWER_CONSUMPTION => Ok(Some(CommandResult::Float32(self.get_consumption()))),
            CMD_SELF_TEST => Ok(Some(CommandResult::Str(String::from("PASSED")))),
            CMD_STATUS => Ok(Some(CommandResult::Str(self.get_status()))),
            CMD_GET_TEMPERATURE => Ok(Some(CommandResult::Float32(self.get_temperature()))),
            _ => return Err(ErrorCode::UnsupportedCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsensor() {
        let mut sensor = TSensor::new(String::from("124"), String::from("IBM"));
        assert!(sensor.get_consumption() == 0.0);
        assert!(sensor.get_temperature().is_nan());
        assert!({
            match sensor.set_power_state(PowerState::ON) {
                Ok(PowerState::ON) => true,
                _ => false,
            }
        });
        assert!(sensor.get_consumption() > 0.0);
        assert!(sensor.get_temperature() > 0.0);
        assert!({
            match sensor.set_power_state(PowerState::OFF) {
                Ok(PowerState::OFF) => true,
                _ => false,
            }
        });
    }

    #[test]
    fn test_tsensor_commands() {
        let mut sensor = TSensor::new(String::from("124"), String::from("IBM"));
        assert!({
            match sensor.set_power_state(PowerState::ON) {
                Ok(PowerState::ON) => true,
                _ => false,
            }
        });
        assert!({
            match sensor.execute_cmd_mut(CMD_SELF_TEST, Option::None) {
                Ok(Some(CommandResult::Str(v))) => v == "PASSED",
                _ => false,
            }
        });
        assert!({
            match sensor.execute_cmd_mut(CMD_STATUS, Option::None) {
                Ok(Some(CommandResult::Str(v))) => {
                    println!("{:?}", v);
                    true
                }
                _ => false,
            }
        });
        assert!({
            match sensor.execute_cmd_mut(CMD_GET_POWER_CONSUMPTION, Option::None) {
                Ok(Some(CommandResult::Float32(v))) => v > 0.0,
                _ => false,
            }
        });
        assert!({
            match sensor.execute_cmd_mut(CMD_GET_TEMPERATURE, Option::None) {
                Ok(Some(CommandResult::Float32(v))) => v > 0.0,
                _ => false,
            }
        });
    }
}
