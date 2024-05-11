use crate::error::DeviceError;
use crate::logical::*;
use crate::physical::*;

/// Default implementation of logical IoT device
/// Logical IoT device is used to define topology of room/home
pub struct Device {
    id: xid::Id,
    class: String,
    physical: Option<Box<dyn PhysicalDevice>>,
}

/// Clone implementation for logical device
impl Clone for Device {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            class: self.class.clone(), // we keep class
            physical: Option::None,    // we don't clone physical device
        }
    }
}

/// IoTDevice defines common API for logical IoT devices
impl Device {
    pub fn new(class: String) -> Self {
        Device {
            id: xid::new(),
            class: class.clone(),
            physical: Option::None,
        }
    }

    pub fn get_id(&self) -> xid::Id {
        self.id
    }

    pub fn get_class(&self) -> String {
        self.class.clone()
    }

    pub fn bind(&mut self, d: Box<dyn crate::physical::PhysicalDevice>) -> Result<(), DeviceError> {
        match self.physical {
            Some(_) => Err(DeviceError::AlreadyBound),
            None => {
                self.physical = Option::Some(d);
                Ok(())
            }
        }
    }

    pub fn unbind(&mut self) -> Result<(), DeviceError> {
        match self.physical {
            None => Ok(()),
            Some(_) => {
                self.physical = Option::None;
                Ok(())
            }
        }
    }

    pub fn is_bound(&self) -> bool {
        self.physical.is_some()
    }

    pub fn get_power_state(&self) -> Result<PowerState, DeviceError> {
        match &self.physical {
            Some(d) => d.get_power_state(),
            None => Err(DeviceError::Unbound),
        }
    }

    pub fn set_power_state(&mut self, s: PowerState) -> Result<PowerState, DeviceError> {
        match &mut self.physical {
            Some(d) => d.set_power_state(s),
            None => Err(DeviceError::Unbound),
        }
    }

    pub fn get_supported_commands(&self) -> Result<Vec<DeviceCommand>, DeviceError> {
        match &self.physical {
            Some(d) => d.get_supported_commands(),
            None => Err(DeviceError::Unbound),
        }
    }

    pub fn execute_cmd_mut(
        &mut self,
        cmd: DeviceCommand,
        ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, DeviceError> {
        match &mut self.physical {
            Some(d) => d.execute_cmd_mut(cmd, ars),
            None => Err(DeviceError::Unbound),
        }
    }

    pub fn execute_cmd(
        &self,
        cmd: DeviceCommand,
        ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, DeviceError> {
        match &self.physical {
            Some(d) => d.execute_cmd(cmd, ars),
            None => Err(DeviceError::Unbound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::CMD_GET_POWER_CONSUMPTION;

    use super::*;

    #[test]
    fn logical_device() {
        let d1 = Device::new(String::from("device A"));
        let d2 = Device::new(String::from("device B"));
        assert_ne!(d1.get_id(), d2.get_id());
        assert_eq!(d1.get_class(), String::from("device A"));
        assert_eq!(d2.get_class(), String::from("device B"));
    }

    #[test]
    fn clone_test() {
        let d1 = Device::new(String::from("device A"));
        let clone = d1.clone();
        assert_eq!(d1.get_id(), clone.get_id());
        assert_eq!(d1.get_class(), clone.get_class());
    }

    #[test]
    fn unbind_test() {
        let d = Device::new(String::from("device A"));
        let mut md = Device::new(String::from("device D"));
        let r = &d;
        assert!(!d.is_bound());
        assert!((match r.get_power_state() {
            Err(DeviceError::Unbound) => Ok(()),
            _ => Err("invalid state"),
        })
        .is_ok());
        assert!((match r.get_supported_commands() {
            Err(DeviceError::Unbound) => Ok(()),
            _ => Err("invalid state"),
        })
        .is_ok());
        assert!((match md.set_power_state(PowerState::ON) {
            Err(DeviceError::Unbound) => Ok(()),
            _ => Err("invalid state"),
        })
        .is_ok());
        assert!(
            (match md.execute_cmd_mut(CMD_GET_POWER_CONSUMPTION, Option::None) {
                Err(DeviceError::Unbound) => Ok(()),
                _ => Err("invalid state"),
            })
            .is_ok()
        )
    }
}
