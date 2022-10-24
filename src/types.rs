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
