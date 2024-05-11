//! Physical module implements physical models for IoT devices

use crate::error::DeviceError;
use crate::logical::*;

/// PhysicalDevice defines common interface for physical IoT devices
pub trait PhysicalDevice {
    /// returns serial number of the device
    fn get_serial(&self) -> String;
    // returns manufactor of the device
    fn get_manufactor(&self) -> String;
    // returns power state of the device
    fn get_power_state(&self) -> Result<PowerState, DeviceError>;
    // set power state of the device
    fn set_power_state(&mut self, state: PowerState) -> Result<PowerState, DeviceError>;
    /// return list of supported command of the device
    fn get_supported_commands(&self) -> Result<Vec<DeviceCommand>, DeviceError>;
    /// execute command on device. Command may take arguments as strings (devices usually talks MQTT, so string is Ok for generic interface)
    fn execute_cmd_mut(
        &mut self,
        cmd: DeviceCommand,
        args: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, DeviceError>;
    /// execute command on device (immutable). Command may take arguments as strings (devices usually talks MQTT, so string is Ok for generic interface)
    fn execute_cmd(
        &self,
        cmd: DeviceCommand,
        ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, DeviceError>;
}
