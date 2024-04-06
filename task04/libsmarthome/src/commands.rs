use crate::logical::*;

/// List of common device commands
pub const CMD_STATUS: DeviceCommand = DeviceCommand(1, "status", "returns device status");
pub const CMD_SELF_TEST: DeviceCommand =
    DeviceCommand(2, "selftest", "performs self test of the device");
pub const CMD_GET_POWER_CONSUMPTION: DeviceCommand = DeviceCommand(
    3,
    "get_power_consumption",
    "returns instance consumption of the device",
);
