#![allow(dead_code)]

use crate::config::QemuConfig;
use crate::device_consts::*;

/// trait that Devices should implement
pub trait Device {
    /// self.valid() returns whether the device can return a valid param format
    fn valid(&self) -> bool;
    /// self.set_qemu_params(config) will plug the param into config
    fn set_qemu_params(&self, config: &mut QemuConfig);
}

/// QEMU object
pub struct Object {
	/// Driver is the qemu device driver
    pub driver: DeviceDriver,

	/// Type is the qemu object type.
    pub obj_type: ObjectType,

	/// ID is the user defined object ID.
    pub id: String,

	/// DeviceID is the user defined device ID.
    pub device_id: String,

	/// MemPath is the object's memory path.
	/// This is only relevant for memory objects
    pub mem_path: String,

	/// Size is the object size in bytes
    pub size: u64,

	/// Debug this is a debug object
    pub debug: bool,

	/// File is the device file
    pub file: String,

	/// FirmwareVolume is the configuration volume for the firmware
	/// it can be used to split the TDVF/OVMF UEFI firmware in UEFI variables
	/// and UEFI program image.
    pub firmware_volume: String,

	/// CBitPos is the location of the C-bit in a guest page table entry
	/// This is only relevant for sev-guest objects
    pub c_bit_pos: u32,

	/// ReducedPhysBits is the reduction in the guest physical address space
	/// This is only relevant for sev-guest objects
    pub reduced_physical_bits: u32,

	/// ReadOnly specifies whether `MemPath` is opened read-only or read/write (default)
    pub rd_only: bool,

	/// Prealloc enables memory preallocation
    pub prealloc: bool,
}

impl Device for Object {
    fn set_qemu_params(&self, _config: &mut QemuConfig) {
        unimplemented!();
    }

    fn valid(&self) -> bool {
        unimplemented!();
    }
}

/// FSDevice represents a qemu filesystem configuration.
pub struct FSDevice {
	/// Driver is the qemu device driver
    pub driver: DeviceDriver,

	/// FSDriver is the filesystem driver backend.
    pub fs_driver: FsDriver,

	/// ID is the filesystem identifier.
    pub id: String,

	/// Path is the host root path for this filesystem.
    pub path: String,

	/// MountTag is the device filesystem mount point tag.
    pub mount_tag: String,

	/// SecurityModel is the security model for this filesystem device.
    pub security_model: SecurityModel,

	/// DisableModern prevents qemu from relying on fast MMIO.
    pub disabled_modern: bool,

	/// ROMFile specifies the ROM file being used for this device.
    pub rom_file: String,

	/// DevNo identifies the ccw devices for s390x architecture
    pub devno: String,

	/// Transport is the virtio transport for this device.
    pub transport: VirtioTransport,

	/// Multidev is the filesystem behaviour to deal
	/// with multiple devices being shared with a 9p export
    pub multidev: Virtio9PMultiDev,
}

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
