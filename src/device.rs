#![allow(dead_code)]

use crate::config::QemuConfig;

/// trait that Devices should implement
pub trait Device {
    /// self.valid() returns whether the device can return a valid param format
    fn valid(&self) -> bool;
    /// self.set_qemu_params(config) will plug the param into config
    fn set_qemu_params(&self, config: &mut QemuConfig);
}

pub struct FSDevice {}

impl Device for FSDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct NetDevice {}

impl Device for NetDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct CharDevice {}

impl Device for CharDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct LegacySerialDevice {}

impl Device for LegacySerialDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct SerialDevice {}

impl Device for SerialDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct BlockDevice {}

impl Device for BlockDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct PVPanicDevice {}

impl Device for PVPanicDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct LoaderDevice {}

impl Device for LoaderDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct VhostUserDevice {}

impl Device for VhostUserDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct PcieRootPortDevice {}

impl Device for PcieRootPortDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct VFIODevice {}

impl Device for VFIODevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct ScsiController {}

impl Device for ScsiController {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct BridgeDevice {}

impl Device for BridgeDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct VSockDevice {}

impl Device for VSockDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct RngDevice {}

impl Device for RngDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct BalloonDevice {}

impl Device for BalloonDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct IommuDevice {}

impl Device for IommuDevice {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

pub struct FwConfig {}

impl Device for FwConfig {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}
