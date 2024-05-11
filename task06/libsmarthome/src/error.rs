use std::{error::Error, fmt::Display};

/// ErrorCode describes error codes of IoT device
#[derive(Debug, Clone)]
pub enum DeviceError {
    /// Operation is not supported by the target
    UnsupportedOperation,
    /// Device is offline
    Off,
    /// No physical device is connected
    Unbound,
    /// Physical device is already bound
    AlreadyBound,
    /// Requested command is unsupported
    UnsupportedCommand,
    /// Command returned unexpected format
    UnexpectedResultFormat,
    /// Command failed, value may include additional details
    CommandFailed(String),
    /// Device is not found in expected place
    DeviceIsMissing(xid::Id),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::UnsupportedOperation => write!(f, "operation is not suported by the target"),
            Self::AlreadyBound => {
                write!(f, "physical device is already bound to this logical device")
            }
            Self::Off => write!(f, "power is off"),
            Self::Unbound => write!(f, "logical device is not bound to physical device"),
            Self::UnsupportedCommand => write!(f, "unsuported command"),
            Self::UnexpectedResultFormat => write!(f, "unexpected result from the command"),
            Self::CommandFailed(v) => write!(f, "command failed with error {}", v),
            Self::DeviceIsMissing(id) => write!(f, "device {} is missing", id),
        }
    }
}

impl Error for DeviceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match &self {
            Self::UnsupportedOperation => "operation is not suported by the target",
            Self::AlreadyBound => "physical device is already bound to this logical device",
            Self::Off => "power is off",
            Self::Unbound => "logical device is not bound to physical device",
            Self::UnsupportedCommand => "unsuported command",
            Self::UnexpectedResultFormat => "unexpected result from the command",
            Self::CommandFailed(_) => "command failed",
            Self::DeviceIsMissing(_) => "device is missing",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
