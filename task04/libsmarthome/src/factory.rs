use crate::logical::IoTDevice;
use crate::logical_device::*;
use crate::physical::*;
use crate::socket::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Describes factory for binding logical device to physical
pub trait PhdFactory {
    fn create_physical_device(&self, d: &Device) -> Result<Box<dyn PhysicalDevice>, ()>;
}

#[derive(Default, Clone, Copy)]
pub struct SimpleClassFactory;

impl PhdFactory for SimpleClassFactory {
    fn create_physical_device(&self, d: &Device) -> Result<Box<dyn PhysicalDevice>, ()> {
        match d.get_class().as_str() {
            "socket" => {
                let socket = ACSocket::new(next_serial_id(), String::from("OTUS"));
                Ok(Box::new(socket))
            }
            _ => Err(()),
        }
    }
}

fn next_serial_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:?}", COUNTER)
}
