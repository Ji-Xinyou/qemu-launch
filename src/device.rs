#![allow(dead_code)]

/// trait that Devices should implement
pub(crate) trait Device {
    fn valid() -> bool;
    fn qemu_params() -> String;
}

struct FSDevice {}

impl Device for FSDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct NetDevice {}

impl Device for NetDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct CharDevice {}

impl Device for CharDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct LegacySerialDevice {}

impl Device for LegacySerialDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct SerialDevice {}

impl Device for SerialDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct BlockDevice {}

impl Device for BlockDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct PVPanicDevice {}

impl Device for PVPanicDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct LoaderDevice {}

impl Device for LoaderDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct VhostUserDevice {}

impl Device for VhostUserDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct PcieRootPortDevice {}

impl Device for PcieRootPortDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct VFIODevice {}

impl Device for VFIODevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct ScsiController {}

impl Device for ScsiController {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct BridgeDevice {}

impl Device for BridgeDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct VSockDevice {}

impl Device for VSockDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct RngDevice {}

impl Device for RngDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct BalloonDevice {}

impl Device for BalloonDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct IommuDevice {}

impl Device for IommuDevice {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}

struct FwConfig {}

impl Device for FwConfig {
    fn qemu_params() -> String {
        unimplemented!();
    }

    fn valid() -> bool {
        unimplemented!();
    }
}
