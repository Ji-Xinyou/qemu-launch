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

            machine_params.push(machine.r#type.clone());

            if !machine.acceleration.is_empty() {
                machine_params.push(format!("accel={}", machine.acceleration.clone()));
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
}
