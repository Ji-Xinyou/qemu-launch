use serde::{Deserialize, Serialize};

/// the machine that qemu will emulate...
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Machine {
    /// machine type used by QEMU
    pub(crate) machine_type: String,

    /// machine acceleration options
    pub(crate) acceleration: String,

    /// options for machine type, e.g. usb=off
    pub(crate) options: String,
}

/// real time clock
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Rtc {
    /// RTC start time
    pub(crate) base: String,

    /// RTC clock driver
    pub(crate) clock: String,

    /// drift fixing mechanism
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
    pub(crate) socket_type: String,

    /// socket name
    pub(crate) name: String,

    /// is socket a server?
    pub(crate) is_server: bool,

    /// if qemu should block waiting for a client to connect
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
    pub(crate) path: String,

    /// guest initrd path on host fs
    pub(crate) initrd_path: String,

    /// guest kernel params
    pub(crate) params: String,
}

/// smp configuration
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Smp {
    /// the number of cpu available to QEMU
    pub(crate) cpus: u32,

    /// the number of cores available to QEMU
    pub(crate) cores: u32,

    /// the number of threads available to QEMU
    pub(crate) threads: u32,

    /// the number of sockets available to QEMU
    pub(crate) sockets: u32,

    /// the maximum number of vcpus to a vm
    /// assert!(max_cpus == 0 || max_cpus >= cpus)
    pub(crate) max_cpus: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// amount of memory available to guest
    /// suffixed with M or G
    pub(crate) size: String,

    /// memory slots available for guest kernel
    pub(crate) slots: u8,

    /// max amount of memory that can be made available to guest
    pub(crate) max_memory: String,

    /// file path for memory device, points to alocal file
    /// used by file backed memory
    pub(crate) path: String,
}
