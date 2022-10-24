use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::device::Device;
use crate::types::{Kernel, Machine, Memory, Smp};

/// the configuration of QEMU
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct QemuConfig {
    /// binary path of QEMU
    pub bin_path: String,

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
    #[serde(skip_deserializing, skip_serializing)]
    devices: Vec<Box<dyn Device>>,

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
    pub qemu_params: Vec<String>,
}

/// QemuConfig
/// # How to build
/// To build your own config, you use builder(), and cumulatively add the components you want
/// ```rust
/// use config::QemuConfig;
///
/// # fn main() {
/// let config = QemuConfig::builder()
///     .add_name("myqemu");
/// # }
/// ```
impl QemuConfig {
    /// From toml, acquires an configuration file.
    /// the configuration did not set the [`qemu_params`] argument, i.e. the function
    /// `build_all(&self)` is not called, instead, all the other fields are filled
    /// In `Qemu::from_config(config: QemuConfig)`, the config.build_all() is called
    /// at there all qemu_params will be filled.
    pub fn from_toml(path: &str) -> Self {
        let content = std::fs::read_to_string(path).expect("failed to fetch file content");
        toml::from_str(&content).expect("failed to get toml content")
    }

    /// Fill the `self.qemu_params` based on the fields we have filled
    /// Notice that this is not idempotent, duplicate call will append
    /// new params after the original ones
    pub fn build_all(&self) -> Self {
        let uuid = Uuid::new_v4();
        let cfg = self.clone();

        cfg.add_cpu_model(&self.cpu_model)
            .add_bios(&self.bios)
            .add_devices(&self.devices)
            .add_global_params(&self.global_params)
            .add_kernel(&self.kernel)
            .add_machine(&self.machine)
            .add_memory(&self.memory)
            .add_name(&self.name)
            .add_seccomp(&self.seccomp_sandbox)
            .add_uuid(uuid)
            .add_smp(&self.smp)
            .expect("failed to build all")
    }

    /// returns a default instance of `QemuConfig`
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_seccomp(mut self, seccomp_sandbox: &str) -> Self {
        if !seccomp_sandbox.is_empty() {
            self.qemu_params.push("-sandbox".to_owned());
            self.qemu_params.push(seccomp_sandbox.to_owned());
        }
        self
    }

    pub fn add_name(mut self, name: &str) -> Self {
        if !name.is_empty() {
            self.qemu_params.push("-name".to_owned());
            self.qemu_params.push(name.to_string());
        }
        self
    }

    pub fn add_machine(mut self, machine: &Machine) -> Self {
        if !machine.machine_type.is_empty() {
            let mut machine_params = vec![];

            machine_params.push(machine.machine_type.to_owned());

            if !machine.acceleration.is_empty() {
                machine_params.push(format!("accel={}", machine.acceleration));
            }

            if !machine.options.is_empty() {
                machine_params.push(machine.options.to_owned());
            }

            self.qemu_params.push("-machine".to_owned());
            self.qemu_params.push(machine_params.join(","));
        }
        self
    }

    pub fn add_cpu_model(mut self, cpu_model: &str) -> Self {
        if !cpu_model.is_empty() {
            self.qemu_params.push("-cpu".to_owned());
            self.qemu_params.push(cpu_model.to_owned());
        }
        self
    }

    /// Normally, we add device after `build_all()` since it is not cloneable
    pub fn add_devices(mut self, devices: &Vec<Box<dyn Device>>) -> Self {
        devices.into_iter().for_each(|dev| {
            if dev.valid() {
                dev.set_qemu_params(&mut self);
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

    pub fn add_memory(mut self, memory: &Memory) -> Self {
        if !memory.size.is_empty() {
            let mut memory_params = vec![];
            memory_params.push(memory.size.to_owned());

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

    pub fn add_smp(mut self, smp: &Smp) -> Result<Self> {
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

            assert_eq!(smp.sockets * smp.cores * smp.threads, smp.max_cpus);

            self.qemu_params.push("-smp".to_owned());
            self.qemu_params.push(smp_params.join(","));
        }
        Ok(self)
    }

    pub fn add_global_params(mut self, global_params: &str) -> Self {
        if !global_params.is_empty() {
            self.qemu_params.push("-global".to_owned());
            self.qemu_params.push(global_params.to_owned());
        }
        self
    }

    pub fn add_kernel(mut self, kernel: &Kernel) -> Self {
        if !kernel.path.is_empty() {
            self.qemu_params.push("-kernel".to_owned());
            self.qemu_params.push(kernel.path.to_owned());

            if !kernel.initrd_path.is_empty() {
                self.qemu_params.push("-initrd".to_owned());
                self.qemu_params.push(kernel.initrd_path.to_owned());
            }

            if !kernel.params.is_empty() {
                self.qemu_params.push("-append".to_owned());
                self.qemu_params.push(kernel.params.to_owned());
            }
        }
        self
    }

    pub fn add_bios(mut self, bios: &str) -> Self {
        if !bios.is_empty() {
            self.qemu_params.push("-bios".to_owned());
            self.qemu_params.push(bios.to_owned());
        }
        self
    }
}

impl QemuConfig {
    pub fn dump(&self) {
        println!("{:?}", self.cpu_model);
    }
}

/// The devices are not cloned, need to re-add the device if cloned
impl Clone for QemuConfig {
    fn clone(&self) -> Self {
        Self {
            bin_path: self.bin_path.clone(),
            uid: self.uid.clone(),
            gid: self.gid.clone(),
            groups: self.groups.clone(),
            name: self.name.clone(),
            uuid: self.uuid.clone(),
            cpu_model: self.cpu_model.clone(),
            seccomp_sandbox: self.seccomp_sandbox.clone(),
            machine: self.machine.clone(),
            devices: vec![],
            vga: self.vga.clone(),
            kernel: self.kernel.clone(),
            memory: self.memory.clone(),
            smp: self.smp.clone(),
            global_params: self.global_params.clone(),
            bios: self.bios.clone(),
            qemu_params: self.qemu_params.clone(),
        }
    }
}
