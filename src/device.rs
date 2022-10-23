#![allow(dead_code)]

/// trait that Devices should implement
pub(crate) trait Device {
    fn valid() -> bool;
    fn qemu_params() -> String;
}

struct FSDevice {}

struct NetDevice {}

struct CharDevice {}

struct LegacySerialDevice {}

struct SerialDevice {}

struct BlockDevice {}

struct PVPanicDevice {}

struct LoaderDevice {}

struct VhostUserDevice {}

struct PcieRootPortDevice {}

struct VFIODevice {}

struct ScsiController {}

struct BridgeDevice {}

struct VSockDevice {}

struct RngDevice {}

struct BalloonDevice {}

struct IommuDevice {}

struct FwConfig {}
