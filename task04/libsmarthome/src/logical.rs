//! Logical module implements logical model of smarthome.

use crate::physical;

/// PowerState describes power state of IoT device
#[derive(Debug, Clone, Copy, Default)]
pub enum PowerState {
    #[default]
    ON,
    OFF,
}

/// ErrorCode describes error codes of IoT device
#[derive(Debug, Clone)]
pub enum ErrorCode {
    /// Device is offline
    Off,
    /// No physical device is connected
    Unbound,
    /// Physical device is already bound
    AlreadyBound,
    /// Requested command is unsupported
    UnsupportedCommand,
    /// Command failed, value may include additional details
    CommandFailed(String),
}

/// DeviceCommand describes command that can be executed by IoT device
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceCommand(pub u64, pub &'static str, pub &'static str);

/// CommandResult describes result of device command
#[derive(Debug, Clone)]
pub enum CommandResult {
    Bool(bool),
    Str(String),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    VecStr(Vec<String>),
    Bytes(Vec<u8>),
}

/// IoTDevice defines common interface for any logical IoT device
pub trait IoTDevice {
    /// return logical ID of the device
    fn get_id(&self) -> xid::Id;
    /// return logical class of the device (sensors, sockets, etc)
    fn get_class(&self) -> String;
    /// binds logical device to physical device
    fn bind(&mut self, d: Box<dyn physical::PhysicalDevice>) -> Result<(), ErrorCode>;
    /// unbinds logical device from physical device
    fn unbind(&mut self) -> Result<(), ErrorCode>;
    /// checks if logical device is bound to physical device
    fn is_bound(&self) -> bool;
    /// return power state of the device
    fn get_power_state(&self) -> Result<PowerState, ErrorCode>;
    /// set power state of the device    
    fn set_power_state(&mut self, s: PowerState) -> Result<PowerState, ErrorCode>;
    /// return list of supported command of the device
    fn get_supported_commands(&self) -> Result<Vec<DeviceCommand>, ErrorCode>;
    /// execute command on device. Command may take arguments as strings (devices usually talks MQTT, so string is Ok for generic interface)
    fn execute_cmd(
        &mut self,
        cmd: DeviceCommand,
        ars: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, ErrorCode>;
}
