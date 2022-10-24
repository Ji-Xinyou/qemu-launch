#![allow(dead_code)]

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
