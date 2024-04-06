//! Physical module implements physical models for IoT devices

use std::error::Error;

use crate::{
    logical::{self, *},
    logical_device::Device,
};

/// PhysicalDevice defines common interface for physical IoT devices
pub trait PhysicalDevice {
    /// returns serial number of the device
    fn get_serial(&self) -> String;
    // returns manufactor of the device
    fn get_manufactor(&self) -> String;
    // returns power state of the device
    fn get_power_state(&self) -> Result<PowerState, ErrorCode>;
    // set power state of the device
    fn set_power_state(&mut self, state: PowerState) -> Result<PowerState, ErrorCode>;
    /// return list of supported command of the device
    fn get_supported_commands(&self) -> Result<Vec<DeviceCommand>, ErrorCode>;
    /// execute command on device. Command may take arguments as strings (devices usually talks MQTT, so string is Ok for generic interface)
    fn execute_cmd(
        &mut self,
        cmd: DeviceCommand,
        ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, ErrorCode>;
}
