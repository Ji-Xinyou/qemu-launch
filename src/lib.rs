#![allow(dead_code)]

pub mod device_consts;
pub mod config;
mod device;
pub mod qemu;
mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
