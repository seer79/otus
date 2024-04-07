pub mod commands;
pub mod factory;
pub mod home;
pub mod logical;
pub mod logical_device;
pub mod physical;
pub mod room;
pub mod socket;
pub mod tsensor;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
