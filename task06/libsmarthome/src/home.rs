use crate::device_ref::*;
use crate::error::DeviceError;
use crate::factory::PhysicalDeviceFactory;
use crate::logical::PowerState;
use crate::report::{Reporter, RoomRef};
use crate::room::*;
use std::collections::{HashMap, HashSet};

/// Home describes smart home
#[derive(Clone, Default)]
pub struct Home {
    /// Home name
    name: String,
    /// Rooms a store as Box<> since they can include many devices
    rooms: HashMap<xid::Id, Box<Room>>,
    /// Help map to find room by label
    label_map: HashMap<String, xid::Id>,
}

impl Home {
    /// Add room to the home
    pub fn add_room(&mut self, room: Box<Room>) -> xid::Id {
        let label = room.get_label();
        let id = room.get_id();
        self.rooms.insert(id, room);
        self.label_map.insert(label, id);
        id
    }

    /// Get mutable reference to the room by id
    pub fn get_room_mut(&mut self, id: &xid::Id) -> Option<&mut Box<Room>> {
        self.rooms.get_mut(id)
    }

    /// Get mutable reference to the room by label
    pub fn get_room_by_lable_mut(&mut self, label: &String) -> Option<&mut Box<Room>> {
        match self.label_map.get(label) {
            None => None,
            Some(id) => self.rooms.get_mut(id),
        }
    }

    /// Remove room from the home by id
    pub fn remove_room(&mut self, id: &xid::Id) -> Option<xid::Id> {
        match self.rooms.remove(id) {
            None => None,
            Some(r) => {
                self.label_map.remove(&r.get_label());
                Some(*id)
            }
        }
    }

    /// Remove room from the home by label
    pub fn remove_room_by_label(&mut self, label: &String) -> Option<xid::Id> {
        match self.label_map.remove(label) {
            None => None,
            Some(id) => self.rooms.remove(&id).map(|r| r.get_id()),
        }
    }

    /// Collect references to devices in home
    pub fn get_devices(&self) -> Vec<DeviceRef> {
        self.rooms.values().flat_map(|r| r.get_devices()).collect()
    }

    /// Check if home has room with given id
    pub fn has_room_with_id(&self, id: &xid::Id) -> bool {
        self.rooms.contains_key(id)
    }

    /// Check if home has room with given label
    pub fn has_room_with_label(&self, l: &String) -> bool {
        self.label_map.contains_key(l)
    }

    /// Switch power of home
    pub fn switch_power(&mut self, state: PowerState) -> Result<(), DeviceError> {
        for room in self.rooms.values_mut() {
            room.switch_power(state)?
        }
        Ok(())
    }

    /// Binds physical devices to logical ones
    pub fn bind_physical_devices<F: PhysicalDeviceFactory>(
        &mut self,
        binder: &mut Binder<F>,
    ) -> Result<(), DeviceError> {
        for room in self.rooms.values_mut() {
            match room.accept_mut(binder) {
                Ok(_) => (),
                Err(v) => {
                    return Err(v);
                }
            }
        }
        Ok(())
    }

    /// Generate status report for devices
    pub fn create_report<T: Reporter>(&self, reporter: &T) -> String {
        let mut report = format!("# IoT report for the home {:?}\n", self.name);
        let mut missings: HashSet<String> = HashSet::default();
        let mut errors: HashSet<String> = HashSet::default();
        let mut room_reports: HashMap<xid::Id, Vec<String>> = HashMap::default();
        reporter
            .get_device_refs()
            .iter()
            .for_each(|dref| match &dref.0 {
                RoomRef::ID(id) => match self.rooms.get(id) {
                    Some(r) => match reporter.get_device_status(r, &dref.1) {
                        Ok(status) => {
                            if !room_reports.contains_key(id) {
                                room_reports.insert(*id, vec![status]);
                            } else {
                                let v = room_reports.get_mut(id).unwrap();
                                v.push(status);
                            }
                        }
                        Err(DeviceError::DeviceIsMissing(id)) => {
                            missings.insert(format!(
                                "ERROR: Device {} is not found in the room {}",
                                id,
                                r.get_label()
                            ));
                        }
                        Err(v) => {
                            errors.insert(format!("ERROR: device {}, err: {:?}", dref.1, v));
                        }
                    },
                    None => {
                        missings.insert(format!("ERROR: Room with ID {} is missing", id));
                    }
                },
                RoomRef::Label(l) => match self.label_map.get(l) {
                    Some(id) => {
                        let room = self.rooms.get(id).unwrap();
                        match reporter.get_device_status(room, &dref.1) {
                            Ok(status) => {
                                if !room_reports.contains_key(id) {
                                    room_reports.insert(*id, vec![status]);
                                } else {
                                    let v = room_reports.get_mut(id).unwrap();
                                    v.push(status);
                                }
                            }
                            Err(DeviceError::DeviceIsMissing(id)) => {
                                missings.insert(format!(
                                    "ERROR: Device {} is not found in the room {}",
                                    id,
                                    room.get_label()
                                ));
                            }
                            Err(v) => {
                                errors.insert(format!("ERROR: device {}, err: {:?}", dref.1, v));
                            }
                        }
                    }
                    None => {
                        missings.insert(format!("Room with label {} is missing", l));
                    }
                },
            });
        let mut room_by_name = self.label_map.keys().collect::<Vec<&String>>();
        room_by_name.sort();
        room_by_name.iter().for_each(|l| {
            let room_id = self.label_map.get(*l).unwrap();
            if let Some(r) = room_reports.get(room_id) {
                report += format!("## Room '{}'\n", l).as_str();
                r.iter().for_each(|s| {
                    report += s;
                    report += "\n";
                })
            }
        });
        report += "## Missing devices\n";
        let mut missing_sorted = missings.iter().collect::<Vec<&String>>();
        missing_sorted.sort();
        missing_sorted.iter().for_each(|s| {
            if !report.is_empty() {
                report += "\n";
            }
            report += s;
        });
        report += "## Errors";
        let mut errors_sorted = errors.iter().collect::<Vec<&String>>();
        errors_sorted.sort();
        errors_sorted.iter().for_each(|s| {
            if !report.is_empty() {
                report += "\n";
            }
            report += s;
        });
        report
    }
}

#[derive(Default)]
pub struct HomeBuilder {
    home: Box<Home>,
}

impl HomeBuilder {
    pub fn new() -> Self {
        Self {
            home: Box::default(),
        }
    }

    pub fn set_name(&mut self, n: String) -> &mut Self {
        self.home.name = n;
        self
    }

    pub fn add_room(&mut self, r: Box<Room>) -> &mut Self {
        let id = r.get_id();
        let label = r.get_label();
        self.home.rooms.insert(id, r);
        self.home.label_map.insert(label.clone(), id);
        self
    }

    pub fn build(&self) -> Box<Home> {
        self.home.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::room::RoomBuilder;

    use super::*;

    #[test]
    fn test_home() {
        let rlabel = String::from("Room A");
        let r = RoomBuilder::new().set_label(rlabel.clone()).build();
        let rid = r.get_id();
        let mut home = HomeBuilder::new()
            .set_name(String::from("My home"))
            .add_room(Box::new(r))
            .build();
        let mut mhome = HomeBuilder::new()
            .add_room(Box::new(
                RoomBuilder::new().set_label(rlabel.clone()).build(),
            ))
            .build();
        assert!(home.has_room_with_id(&rid));
        assert!(home.has_room_with_label(&rlabel));
        assert!(mhome.switch_power(PowerState::ON).is_ok());
        let id = home.add_room(Box::new(
            RoomBuilder::new()
                .set_label("added room".to_string())
                .build(),
        ));
        assert!(home.has_room_with_label(&"added room".to_string()));
        assert!(home.remove_room(&id).is_some());
        assert!(!home.has_room_with_label(&"added room".to_string()));
        home.add_room(Box::new(
            RoomBuilder::new()
                .set_label("added room 2".to_string())
                .build(),
        ));
        assert!(home.has_room_with_label(&"added room 2".to_string()));
        assert!(home
            .remove_room_by_label(&"added room 2".to_string())
            .is_some());
    }
}
