use uuid::Uuid;
use anyhow::{anyhow, Result};

use crate::device::Device;
use crate::types::{Kernel, Machine, Memory, Smp};

/// the configuration of QEMU
#[derive(Debug, Default)]
pub struct QemuConfig {
    /// binary path of QEMU
    bin_path: String,

    /// user id
    uid: u32,

    /// group id
    gid: u32,

    /// groups(supplementary group IDs)
    groups: Vec<u32>,

    /// QEMU guest name
    name: String,

    /// uuid of qemu process
    uuid: String,

    /// cpu model used by QEMU
    cpu_model: String,

    /// qemu function which enables the seccomp feature
    seccomp_sandbox: String,

    /// machine type configuration
    machine: Machine,

    // todo: qmp socket
    // todo: RTC(real-time-clock)
    /// vga mode
    vga: String,

    /// guest kernel configuration
    kernel: Kernel,

    /// guest memory configuration
    memory: Memory,

    /// guest mp configuration
    smp: Smp,

    /// -global
    global_params: String,

    // todo: knobs

    // -bios
    bios: String,

    // todo: pflash
    // todo: incoming
    // todo: fds
    // todo: -fw_cfg
    // todo: iothreads
    // todo: pidfile
    // todo: logfile
    /// qemu parameters
    qemu_params: Vec<String>,
}

/// QemuConfig builder
impl QemuConfig {
    /// returns a default configuration
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_seccomp(mut self, seccomp_sandbox: String) -> Self {
        if !seccomp_sandbox.is_empty() {
            self.qemu_params.push("-sandbox".to_owned());
            self.qemu_params.push(seccomp_sandbox);
        }
        self
    }

    pub fn add_name(mut self, name: String) -> Self {
        if !name.is_empty() {
            self.qemu_params.push("-name".to_owned());
            self.qemu_params.push(name);
        }
        self
    }

    pub fn add_machine(mut self, machine: Machine) -> Self {
        if !machine.r#type.is_empty() {
            let mut machine_params = vec![];

            machine_params.push(machine.r#type);

            if !machine.acceleration.is_empty() {
                machine_params.push(format!("accel={}", machine.acceleration));
            }

            if !machine.options.is_empty() {
                machine_params.push(machine.options);
            }

            self.qemu_params.push("-machine".to_owned());
            self.qemu_params.push(machine_params.join(","));
        }
        self
    }

    pub fn add_cpu_model(mut self, cpu_model: String) -> Self {
        if !cpu_model.is_empty() {
            self.qemu_params.push("-cpu".to_owned());
            self.qemu_params.push(cpu_model);
        }
        self
    }

    pub fn add_devices(mut self, devices: Vec<Box<dyn Device>>) -> Self {
        devices.into_iter().for_each(|dev| {
            if dev.valid() {
                dev.set_qemu_params(&self);
            }
        });
        self
    }

    pub fn add_uuid(mut self, uuid: Uuid) -> Self {
        if !uuid.is_nil() {
            self.qemu_params.push("-uuid".to_owned());
            self.qemu_params.push(uuid.to_string());
        }
        self
    }

    pub fn add_memory(mut self, memory: Memory) -> Self {
        if !memory.size.is_empty() {
            let mut memory_params = vec![];
            memory_params.push(memory.size);

            if memory.slots > 0 {
                memory_params.push(format!("slots={}", memory.slots));
            }

            if !memory.max_memory.is_empty() {
                memory_params.push(format!("maxmem={}", memory.max_memory));
            }

            self.qemu_params.push("-m".to_owned());
            self.qemu_params.push(memory_params.join(","));
        }
        self
    }

    pub fn add_smp(mut self, smp: Smp) -> Result<Self> {
        if smp.cpus > 0 {
            let mut smp_params = vec![];
            smp_params.push(smp.cpus.to_string());

            if smp.cores > 0 {
                smp_params.push(format!("cores={}", smp.cores));
            }

            if smp.threads > 0 {
                smp_params.push(format!("threads={}", smp.threads));
            }

            if smp.sockets > 0 {
                smp_params.push(format!("sockets={}", smp.sockets));
            }

            if smp.max_cpus > 0 {
                if smp.max_cpus < smp.cpus {
                    return Err(anyhow!("smp.max_cpus should >= smp.cpus"));
                }
                smp_params.push(format!("maxcpus={}", smp.max_cpus));
            }

            self.qemu_params.push("-smp".to_owned());
            self.qemu_params.push(smp_params.join(","));
        }
        Ok(self)
    }

    pub fn add_global_params(mut self, global_params: String) -> Self {
        if !global_params.is_empty() {
            self.qemu_params.push("-global".to_owned());
            self.qemu_params.push(global_params);
        }
        self
    }

    pub fn add_kernel(mut self, kernel: Kernel) -> Self {
        if !kernel.path.is_empty() {
            self.qemu_params.push("-kernel".to_owned());
            self.qemu_params.push(kernel.path);

            if !kernel.initrd_path.is_empty() {
                self.qemu_params.push("-initrd".to_owned());
                self.qemu_params.push(kernel.initrd_path);
            }

            if !kernel.params.is_empty() {
                self.qemu_params.push("-append".to_owned());
                self.qemu_params.push(kernel.params);
            }
        }
        self
    }

    pub fn add_bios(mut self, bios: String) -> Self {
        if !bios.is_empty() {
            self.qemu_params.push("-bios".to_owned());
            self.qemu_params.push(bios);
        }
        self
    }
}
