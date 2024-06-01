use rand::Rng;

pub mod iotserver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    OFF,
    ON,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Commands {
    PowerOn = 1,
    PowerOff = 2,
    GetStatus = 3,
    GetConsumption = 4,
    ListDevices = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACSocket {
    state: PowerState,
    id: xid::Id,
}

impl ACSocket {
    pub fn new() -> Self {
        Self {
            state: PowerState::OFF,
            id: xid::new(),
        }
    }

    pub fn get_id(&self) -> xid::Id {
        self.id
    }

    pub fn switch(&mut self, state: PowerState) -> &mut Self {
        self.state = state;
        self
    }

    pub fn get_consumption(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.1..100.0)
    }

    pub fn get_state(&self) -> String {
        format!(
            "power = {:?}, consumption = {}",
            self.state,
            self.get_consumption()
        )
    }
}
