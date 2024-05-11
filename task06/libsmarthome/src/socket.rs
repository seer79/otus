use crate::commands::CMD_GET_POWER_CONSUMPTION;
use crate::commands::CMD_SELF_TEST;
use crate::commands::CMD_STATUS;
use crate::error::DeviceError;
use crate::logical;
use crate::logical::*;
use crate::physical::*;
use rand::Rng;

/// ACSocket describes AC socket
pub struct ACSocket {
    serial: String,
    manufactor: String,
    state: logical::PowerState,
}

impl ACSocket {
    pub fn new(serial: String, manufactor: String) -> Self {
        ACSocket {
            serial,
            manufactor,
            state: PowerState::OFF,
        }
    }

    fn get_consumption(&self) -> f32 {
        match &self.state {
            PowerState::OFF => 0.0,
            PowerState::ON => {
                let mut rng = rand::thread_rng();
                rng.gen_range(0.1..100.0)
            }
        }
    }

    fn get_status(&self) -> String {
        format!(
            "AC Socket: serial = {}, manufactor = {}, power state = {:?}, consumption = {}",
            self.serial,
            self.manufactor,
            self.state,
            self.get_consumption()
        )
    }
}

impl PhysicalDevice for ACSocket {
    fn get_serial(&self) -> String {
        self.serial.clone()
    }

    fn get_manufactor(&self) -> String {
        self.manufactor.clone()
    }

    fn get_power_state(&self) -> Result<PowerState, DeviceError> {
        Ok(self.state)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<PowerState, DeviceError> {
        self.state = state;
        Ok(self.state)
    }

    fn get_supported_commands(&self) -> Result<Vec<DeviceCommand>, DeviceError> {
        Ok(vec![CMD_STATUS, CMD_GET_POWER_CONSUMPTION, CMD_SELF_TEST])
    }

    fn execute_cmd_mut(
        &mut self,
        cmd: DeviceCommand,
        _ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, DeviceError> {
        match cmd {
            CMD_GET_POWER_CONSUMPTION => Ok(Some(CommandResult::Float32(self.get_consumption()))),
            CMD_SELF_TEST => Ok(Some(CommandResult::Str(String::from("PASSED")))),
            CMD_STATUS => Ok(Some(CommandResult::Str(self.get_status()))),
            _ => Err(DeviceError::UnsupportedCommand),
        }
    }

    fn execute_cmd(
        &self,
        cmd: DeviceCommand,
        _ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, DeviceError> {
        match cmd {
            CMD_GET_POWER_CONSUMPTION => Ok(Some(CommandResult::Float32(self.get_consumption()))),
            CMD_SELF_TEST => Ok(Some(CommandResult::Str(String::from("PASSED")))),
            CMD_STATUS => Ok(Some(CommandResult::Str(self.get_status()))),
            _ => Err(DeviceError::UnsupportedCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acsocket() {
        let mut socket = ACSocket::new(String::from("124"), String::from("IBM"));
        assert!(socket.get_consumption() == 0.0);
        assert!(matches!(
            socket.set_power_state(PowerState::ON),
            Ok(PowerState::ON)
        ));
        assert!(socket.get_consumption() > 0.0);
        assert!(matches!(
            socket.set_power_state(PowerState::OFF),
            Ok(PowerState::OFF)
        ));
    }

    #[test]
    fn test_ac_commands() {
        let mut socket = ACSocket::new(String::from("124"), String::from("IBM"));
        assert!(matches!(
            socket.set_power_state(PowerState::ON),
            Ok(PowerState::ON)
        ));
        assert!(matches!(
            socket.execute_cmd_mut(CMD_SELF_TEST, Option::None),
            Ok(Some(CommandResult::Str(_)))
        ));
        assert!({
            match socket.execute_cmd_mut(CMD_STATUS, Option::None) {
                Ok(Some(CommandResult::Str(v))) => {
                    println!("{:?}", v);
                    true
                }
                _ => false,
            }
        });
        assert!({
            match socket.execute_cmd_mut(CMD_GET_POWER_CONSUMPTION, Option::None) {
                Ok(Some(CommandResult::Float32(v))) => v > 0.0,
                _ => false,
            }
        });
    }
}
