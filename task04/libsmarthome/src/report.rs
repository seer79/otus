//! Defines module for report generation

use crate::commands::*;
use crate::logical::*;
use crate::room::*;

/// Describes reference to room: either by ID or by Label
#[derive(Clone, Debug)]
pub enum RoomRef {
    ID(xid::Id),
    Label(String),
}

/// Describes reference to device. Reference consists of reference to room and device ID
#[derive(Clone, Debug)]
pub struct DeviceRef(pub RoomRef, pub xid::Id);

/// Describes device info collector
pub trait Reporter {
    /// returns list of included devices in the report
    fn get_device_refs(&self) -> Vec<DeviceRef>;

    /// generates report for the device
    fn get_device_status(&self, room: &Room, device_id: &xid::Id) -> Result<String, ErrorCode>;
}

/// Simple reporter based on list of device refs and STATUS command
#[derive(Debug, Default)]
pub struct SimpleReporter {
    refs: Vec<DeviceRef>,
}

impl Reporter for SimpleReporter {
    fn get_device_refs(&self) -> Vec<DeviceRef> {
        self.refs.clone()
    }

    fn get_device_status(&self, room: &Room, device_id: &xid::Id) -> Result<String, ErrorCode> {
        match room.send_cmd(device_id, CMD_STATUS, Option::None) {
            Ok(Some(CommandResult::Str(status))) => Ok(status),
            Err(v) => Err(v),
            _ => Err(ErrorCode::UnexpectedResultFormat),
        }
    }
}

impl SimpleReporter {
    pub fn add_device(&mut self, room_id: &xid::Id, device_id: &xid::Id) {
        self.refs.push(DeviceRef(RoomRef::ID(*room_id), *device_id));
    }
}
