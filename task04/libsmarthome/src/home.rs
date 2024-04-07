use std::collections::HashMap;

use crate::logical::{ErrorCode, PowerState};
use crate::room::*;

/// Home describes smart home
#[derive(Clone, Default)]
pub struct Home {
    /// Rooms a store as Box<> since they can include many devices
    rooms: HashMap<xid::Id, Box<Room>>,
    /// Help map to find room by label
    label_map: HashMap<String, xid::Id>,
}

impl Home {
    /// Check if home has room with given id
    pub fn has_room_with_id(&self, id: &xid::Id) -> bool {
        self.rooms.contains_key(id)
    }

    /// Check if home has room with given label
    pub fn has_room_with_label(&self, l: &String) -> bool {
        self.label_map.contains_key(l)
    }

    /// Switch power of home
    pub fn switch_power(&mut self, state: PowerState) -> Result<(), ErrorCode> {
        for room in self.rooms.values_mut() {
            match room.switch_power(state) {
                Err(v) => return Err(v),
                Ok(_) => (),
            }
        }
        Ok(())
    }
}

pub struct HomeBuilder {
    home: Box<Home>,
}

impl HomeBuilder {
    pub fn new() -> Self {
        Self {
            home: Box::new(Home::default()),
        }
    }

    pub fn add_room(&mut self, r: Box<Room>) -> &mut Self {
        let id = r.get_id();
        let label = r.get_label();
        self.home.rooms.insert(id.clone(), r);
        self.home.label_map.insert(label.clone(), id.clone());
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
        let home = HomeBuilder::new().add_room(Box::new(r)).build();
        let mut mhome = HomeBuilder::new()
            .add_room(Box::new(
                RoomBuilder::new().set_label(rlabel.clone()).build(),
            ))
            .build();
        assert!(home.has_room_with_id(&rid));
        assert!(home.has_room_with_label(&rlabel));
        assert!(mhome.switch_power(PowerState::ON).is_ok());
    }
}
