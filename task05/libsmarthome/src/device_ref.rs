use std::fmt::Display;

/// Describe reference to device in the home: <room id, room label, device id, device class>
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceRef(xid::Id, String, xid::Id, String);

impl DeviceRef {
    pub fn new(
        room_id: xid::Id,
        room_name: String,
        dev_id: xid::Id,
        dev_class: String,
    ) -> DeviceRef {
        DeviceRef(room_id, room_name, dev_id, dev_class)
    }

    pub fn room_id(&self) -> &xid::Id {
        &self.0
    }
    pub fn room_name(&self) -> &String {
        &self.1
    }
    pub fn device_id(&self) -> &xid::Id {
        &self.2
    }
    pub fn device_class(&self) -> &String {
        &self.3
    }
}

impl Display for DeviceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "room id = {}, room name = {}, device id = {}, device class = {}",
            self.0, self.1, self.2, self.3
        )
    }
}
