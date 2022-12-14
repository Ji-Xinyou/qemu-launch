use std::os::unix::prelude::RawFd;

use crate::config::QemuConfig;
use serde::{Deserialize, Serialize};

pub(crate) const MIGRATION_FD: &str = "fd";
pub(crate) const MIGRATION_EXEC: &str = "exec";
pub(crate) const MIGRATION_DEFER: &str = "defer";
pub(crate) const MACHINE_TYPE_MICROVM: &str = "microvm";

/// the machine that qemu will emulate...
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Machine {
    /// machine type used by QEMU
    #[serde(default)]
    pub(crate) machine_type: String,

    /// machine acceleration options
    #[serde(default)]
    pub(crate) acceleration: String,

    /// options for machine type, e.g. usb=off
    #[serde(default)]
    pub(crate) options: String,
}

/// real time clock
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Rtc {
    /// RTC start time
    #[serde(default)]
    pub(crate) base: String,

    /// RTC clock driver
    #[serde(default)]
    pub(crate) clock: String,

    /// drift fixing mechanism
    #[serde(default)]
    pub(crate) drift_fix: String,
}

impl Rtc {
    pub(crate) fn valid(&self) -> bool {
        const HOST: &str = "host";
        const RT: &str = "rt";
        const VM: &str = "vm";
        const SLEW: &str = "slew";
        const NODRIFTFIX: &str = "none";

        let clock_valid = (self.clock == HOST) || (self.clock == RT) || (self.clock == VM);
        let drift_fix_valid = (self.drift_fix == SLEW) || (self.drift_fix == NODRIFTFIX);
        clock_valid && drift_fix_valid
    }
}

/// QMP socket
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct QmpSocket {
    /// the socket's type, unix, hvsock, etc.
    #[serde(default)]
    pub(crate) socket_type: String,

    /// socket name
    #[serde(default)]
    pub(crate) name: String,

    /// is socket a server?
    #[serde(default)]
    pub(crate) is_server: bool,

    /// if qemu should block waiting for a client to connect
    #[serde(default)]
    pub(crate) no_wait: bool,
}

impl QmpSocket {
    pub(crate) fn valid(&self) -> bool {
        const UNIX_SOCKET: &str = "unix";

        if self.socket_type.is_empty() || self.name.is_empty() {
            return false;
        }

        if self.socket_type != UNIX_SOCKET {
            return false;
        }

        true
    }
}

/// the kernel qemu runs
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Kernel {
    /// guest kernel path on host fs
    #[serde(default)]
    pub(crate) path: String,

    /// guest initrd path on host fs
    #[serde(default)]
    pub(crate) initrd_path: String,

    /// guest kernel params
    #[serde(default)]
    pub(crate) params: String,
}

/// smp configuration
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Smp {
    /// the number of cpu available to QEMU
    #[serde(default)]
    pub(crate) cpus: u32,

    /// the number of cores available to QEMU
    #[serde(default)]
    pub(crate) cores: u32,

    /// the number of threads available to QEMU
    #[serde(default)]
    pub(crate) threads: u32,

    /// the number of sockets available to QEMU
    #[serde(default)]
    pub(crate) sockets: u32,

    /// the maximum number of vcpus to a vm
    /// assert!(max_cpus == 0 || max_cpus >= cpus)
    #[serde(default)]
    pub(crate) max_cpus: u32,
}

/// qemu VM memory setups
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// amount of memory available to guest
    /// suffixed with M or G
    #[serde(default)]
    pub(crate) size: String,

    /// memory slots available for guest kernel
    #[serde(default)]
    pub(crate) slots: u8,

    /// max amount of memory that can be made available to guest
    #[serde(default)]
    pub(crate) max_memory: String,

    /// file path for memory device, points to alocal file
    /// used by file backed memory
    #[serde(default)]
    pub(crate) path: String,
}

/// Regroups a set of qemu boolean setups
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Knobs {
    /// prevents qemu from loading user config files
    #[serde(default)]
    pub(crate) no_user_config: bool,

    /// prevents qemu from creating default devices
    #[serde(default)]
    pub(crate) no_defaults: bool,

    /// disable graphic output
    #[serde(default)]
    pub(crate) no_graphic: bool,

    /// turn qemu process into a daemon
    #[serde(default)]
    pub(crate) demonized: bool,

    /// Both hugepages and mem_prealloc require the Memory.size of the VM
    /// to be set, as they need to reserve the memory upfront in order
    /// to let the VM boot without errors
    ///
    /// hugepages always result in memory pre-allocation.
    /// However, the setup is different from normal pre-allocation.
    /// Hence hugepages has precedence over mem_prealloc, and will preallocate
    /// all the RAM from huge pages
    #[serde(default)]
    pub(crate) hugepages: bool,

    /// allocate all memory upfront
    #[serde(default)]
    pub(crate) mem_prealloc: bool,

    /// requires Memory.size and Memory.Path to be set
    #[serde(default)]
    pub(crate) file_backed_mem: bool,

    /// set the memory device as shared
    #[serde(default)]
    pub(crate) mem_shared: bool,

    /// control locking of memory, with this option,
    /// qemu can pin down guest and qemu memory before bootng guest,
    /// i.e. host will not swap them out
    #[serde(default)]
    pub(crate) mlock: bool,

    /// do not start guest CPU at startup
    #[serde(default)]
    pub(crate) stopped: bool,

    /// exit instead of reboot, prevent from rebooting in the event of
    /// triple fault
    #[serde(default)]
    pub(crate) no_reboot: bool,

    /// do not exit qemu on guest shutdown, only stop emulation
    #[serde(default)]
    pub(crate) no_shutdown: bool,

    /// enable iommu for supported devices
    #[serde(default)]
    pub(crate) iommu_platform: bool,
}

/// Allows IO to be performed on a separated thread
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IoThread {
    #[serde(default)]
    pub(crate) id: String,
}

/// controls qemu live migration source preparation
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Incoming {
    /// possible migration types are "fd", "exec", "defer"
    #[serde(default)]
    pub(crate) migration_type: String,

    /// only valid if migration type is "fd"
    #[serde(default)]
    pub(crate) fd: RawFd,

    /// only valid if migration type if "exec"
    #[serde(default)]
    pub(crate) exec: String,
}

/// firmware config allows qemu to pass entries to the guest, could be found under sysfs
/// file and str are mutually exclusive
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FwCfg {
    #[serde(default)]
    pub(crate) name: String,

    #[serde(default)]
    pub(crate) file: String,

    #[serde(default)]
    pub(crate) str: String,
}

impl FwCfg {
    /// returns whether a fwcfg is valid, and can be used
    pub(crate) fn valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }

        if !self.file.is_empty() && !self.str.is_empty() {
            return false;
        }

        if self.file.is_empty() && self.str.is_empty() {
            return false;
        }

        true
    }

    /// setup fwcfg's qemu params
    pub(crate) fn qemu_params(&self, config: &mut QemuConfig) {
        let mut fw_cfg_params = vec![];
        if !self.name.is_empty() {
            fw_cfg_params.push(format!("name={}", &self.name));
        }

        if !self.file.is_empty() {
            fw_cfg_params.push(format!("file={}", &self.file));
        }

        if !self.str.is_empty() {
            fw_cfg_params.push(format!("string={}", &self.str));
        }

        config.qemu_params.push("-fw_cfg".to_owned());
        config.qemu_params.push(fw_cfg_params.join(","));
    }
}

