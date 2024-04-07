//! This module describes room of smart home

use std::collections::HashMap;

use crate::factory::*;
use crate::logical::*;
use crate::logical_device::*;
use xid;

#[derive(Clone)]
pub struct Room {
    id: xid::Id,
    label: String,
    devices: HashMap<xid::Id, Device>,
}

/// Describe API for device visitor of home room
pub trait DeviceVisitor {
    fn accept_mut(&self, d: &mut Device) -> Result<bool, ErrorCode>;

    fn accept(&self, d: &Device) -> Result<bool, ErrorCode>;
}

/// Defines builder pattern for Room
pub struct RoomBuilder {
    room: Room,
}

/// Room implementation
impl Room {
    pub fn new() -> Self {
        Self {
            id: xid::new(),
            label: String::default(),
            devices: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> xid::Id {
        self.id
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn set_label(&mut self, l: String) -> &mut Self {
        self.label = l;
        self
    }

    pub fn get_display_name(&self) -> String {
        if self.label.is_empty() {
            self.id.to_string()
        } else {
            self.label.clone()
        }
    }

    pub fn has_device(&self, id: xid::Id) -> bool {
        self.devices.contains_key(&id)
    }

    pub fn add_device(&mut self, d: Device) -> &mut Self {
        self.devices.insert(d.get_id(), d);
        self
    }

    pub fn remove_device(&mut self, id: &xid::Id) -> &mut Self {
        self.devices.remove(id);
        self
    }

    /// Set power state for devices in the room
    pub fn switch_power(&mut self, state: PowerState) -> Result<(), ErrorCode> {
        for d in self.devices.values_mut() {
            match d.set_power_state(state) {
                Ok(_) => (),
                Err(v) => return Err(v),
            }
        }
        Ok(())
    }

    /// send command to the device in room
    pub fn send_cmd(
        &self,
        &device_id: &xid::Id,
        cmd: DeviceCommand,
        args: Option<Vec<String>>,
    ) -> Result<Option<CommandResult>, ErrorCode> {
        match self.devices.get(&device_id) {
            Some(d) => d.execute_cmd(cmd, args),
            None => Err(ErrorCode::DeviceIsMissing(device_id.clone())),
        }
    }

    /// Mutable visitor
    pub fn accept_mut<T: DeviceVisitor>(&mut self, visitor: &T) -> Result<(), ErrorCode> {
        for v in self.devices.values_mut() {
            match visitor.accept_mut(v) {
                Ok(true) => (),             // continue iteration
                Ok(false) => return Ok(()), // break iteration
                Err(v) => return Err(v),    // some error with device
            }
        }
        Ok(())
    }

    /// Immutable visitor
    pub fn accept<T: DeviceVisitor>(&self, visitor: &T) -> Result<(), ErrorCode> {
        for v in self.devices.values() {
            match visitor.accept(v) {
                Ok(true) => (),             // continue iteration
                Ok(false) => return Ok(()), // break iteration
                Err(v) => return Err(v),    // some error with device
            }
        }
        Ok(())
    }
}

impl RoomBuilder {
    pub fn new() -> Self {
        Self { room: Room::new() }
    }

    pub fn add_device(&mut self, d: &Device) -> &mut Self {
        self.room.add_device(d.clone());
        self
    }

    pub fn set_label(&mut self, l: String) -> &mut Self {
        self.room.set_label(l);
        self
    }

    pub fn build(&self) -> Room {
        self.room.clone()
    }
}

/// Binds logical devices to physical in the room
pub struct Binder<F: PhysicalDeviceFactory> {
    factory: F,
}

impl<F: PhysicalDeviceFactory> Binder<F> {
    pub fn new(f: F) -> Self {
        Self { factory: f }
    }
}

impl<F: PhysicalDeviceFactory> DeviceVisitor for Binder<F> {
    fn accept_mut(&self, d: &mut Device) -> Result<bool, ErrorCode> {
        match self.factory.create_physical_device(d) {
            Ok(pd) => match d.bind(pd) {
                Ok(_) => Ok(true),
                Err(v) => Err(v),
            },
            Err(_) => Err(ErrorCode::UnsupportedOperation),
        }
    }

    fn accept(&self, _d: &Device) -> Result<bool, ErrorCode> {
        Err(ErrorCode::UnsupportedOperation) // client must use mutable version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_builder() {
        let r1 = RoomBuilder::new().set_label(String::from("aa")).build();
        assert_eq!(r1.get_label(), "aa");
        let socket1 = Device::new(String::from("socket"));
        let socket2 = Device::new(String::from("socket"));
        let r2 = RoomBuilder::new()
            .set_label(String::from("aa"))
            .add_device(&socket1)
            .add_device(&socket2)
            .build();
        assert!(r2.has_device(socket1.get_id()));
        assert!(r2.has_device(socket2.get_id()));
    }

    #[test]
    fn test_binder() {
        let socket1 = Device::new(String::from("socket"));
        let socket2 = Device::new(String::from("socket"));
        let factory = SimpleClassFactory::default();
        let binder = Binder::new(factory);
        let mut room = RoomBuilder::new()
            .set_label(String::from("aa"))
            .add_device(&socket1)
            .add_device(&socket2)
            .build();
        assert!(room.accept_mut(&binder).is_ok());
        assert!(room.devices.get(&socket1.get_id()).unwrap().is_bound());
    }
}
