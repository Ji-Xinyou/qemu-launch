use std::os::unix::prelude::RawFd;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::device::Device;
use crate::types::{Incoming, IoThread, Kernel, Knobs, Machine, Memory, QmpSocket, Rtc, Smp, FwCfg};
use crate::types::{MACHINE_TYPE_MICROVM, MIGRATION_DEFER, MIGRATION_EXEC, MIGRATION_FD};

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

    qmp_sockets: Vec<QmpSocket>,

    #[serde(skip_deserializing, skip_serializing)]
    devices: Vec<Box<dyn Device>>,

    rtc: Rtc,

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

    knobs: Knobs,

    // -bios
    bios: String,

    no_graphic: bool,

    pflashs: Vec<String>,

    incoming: Incoming,

    fds: Vec<RawFd>,

    fw_cfgs: Vec<FwCfg>,

    io_threads: Vec<IoThread>,

    pid_file: String,

    log_file: String,

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

        // the order of the functions matters
        let cfg = cfg
            .add_cpu_model(&self.cpu_model)
            .add_bios(&self.bios)
            .add_kernel(&self.kernel)
            .add_machine(&self.machine)
            .add_memory(&self.memory)
            .add_name(&self.name)
            .add_seccomp(&self.seccomp_sandbox)
            .add_uuid(uuid)
            .add_no_graphic(self.no_graphic)
            .add_rtc(&self.rtc)
            .add_qmp_sockets(&self.qmp_sockets)
            .add_vga(&self.vga)
            .add_io_threads(&self.io_threads)
            .add_incoming(&self.incoming)
            .add_pflash_param(&self.pflashs)
            .add_pid_file(&self.pid_file)
            .add_log_file(&self.log_file)
            .add_global_params(&self.global_params)
            .add_knobs(&self.knobs)
            .add_smp(&self.smp)
            .expect("failed to build all");

        // call add_devices after regular appendance
        cfg.add_devices(&self.devices)
    }

    /// returns a default instance of `QemuConfig`
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// setup the seccomp
    pub fn add_seccomp(mut self, seccomp_sandbox: &str) -> Self {
        if !seccomp_sandbox.is_empty() {
            self.qemu_params.push("-sandbox".to_owned());
            self.qemu_params.push(seccomp_sandbox.to_owned());
        }
        self
    }

    /// setup the name of qemu process
    pub fn add_name(mut self, name: &str) -> Self {
        if !name.is_empty() {
            self.qemu_params.push("-name".to_owned());
            self.qemu_params.push(name.to_string());
        }
        self
    }

    /// setup the machine type and related settings, e.g. accel=kvm
    pub fn add_machine(mut self, machine: &Machine) -> Self {
        if !machine.machine_type.is_empty() {
            let mut machine_params = vec![machine.machine_type.to_owned()];
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

    /// setup the cpu model that qemu emulates
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

    /// setup the uuid of qemu
    pub fn add_uuid(mut self, uuid: Uuid) -> Self {
        if !uuid.is_nil() {
            self.qemu_params.push("-uuid".to_owned());
            self.qemu_params.push(uuid.to_string());
        }
        self
    }

    /// setup the memory for VM
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

    /// setup the CPU configuration for VM
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

    /// add global params
    pub fn add_global_params(mut self, global_params: &str) -> Self {
        if !global_params.is_empty() {
            self.qemu_params.push("-global".to_owned());
            self.qemu_params.push(global_params.to_owned());
        }
        self
    }

    /// setup kernel, init ramdisk, and other params, e.g. -append "root=/dev/vda console=ttyS0"
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

    /// setup the bios that qemu uses
    pub fn add_bios(mut self, bios: &str) -> Self {
        if !bios.is_empty() {
            self.qemu_params.push("-bios".to_owned());
            self.qemu_params.push(bios.to_owned());
        }
        self
    }

    /// disable the graphical output
    pub fn add_no_graphic(mut self, no_graphic: bool) -> Self {
        if no_graphic {
            self.qemu_params.push("-nographic".to_owned());
        }
        self
    }

    /// setup the real time clock of qemu
    pub fn add_rtc(mut self, rtc: &Rtc) -> Self {
        if !rtc.valid() {
            return self;
        }

        let mut rtc_params = vec![format!("base={}", rtc.base)];

        if !rtc.drift_fix.is_empty() {
            rtc_params.push(format!("driftfix={}", rtc.drift_fix));
        }

        if !rtc.clock.is_empty() {
            rtc_params.push(format!("clock={}", rtc.clock));
        }

        self.qemu_params.push(rtc_params.join(","));
        self
    }

    /// add qmp sockets to qemu
    pub fn add_qmp_sockets(mut self, qmp_sockets: &Vec<QmpSocket>) -> Self {
        for socket in qmp_sockets {
            if !socket.valid() {
                continue;
            }

            let mut qmp_params = vec![format!("{}:{}", socket.socket_type, socket.name)];
            if socket.is_server {
                qmp_params.push("server=on".to_owned());
                if socket.no_wait {
                    qmp_params.push("wait=off".to_owned());
                }
            }

            self.qemu_params.push("-qmp".to_owned());
            self.qemu_params.push(qmp_params.join(","))
        }
        self
    }

    /// setup the vga for qemu
    pub fn add_vga(mut self, vga: &str) -> Self {
        if !vga.is_empty() {
            self.qemu_params.push("-vga".to_owned());
            self.qemu_params.push(vga.to_owned());
        }
        self
    }

    /// XXX: ONLY called AFTER add_memory() and machine_type is set
    /// setup the boolean configurations
    pub fn add_knobs(mut self, knobs: &Knobs) -> Self {
        if knobs.no_user_config {
            self.qemu_params.push("-no-user-config".to_owned());
        }

        if knobs.no_reboot {
            self.qemu_params.push("--no-reboot".to_owned());
        }

        if knobs.no_graphic {
            self.qemu_params.push("-nographic".to_owned());
        }

        if knobs.no_defaults {
            self.qemu_params.push("-nodefaults".to_owned());
        }

        if knobs.no_shutdown {
            self.qemu_params.push("--no-shutdown".to_owned());
        }

        if knobs.demonized {
            self.qemu_params.push("-daemonize".to_owned());
        }

        self.add_knobs_memory(knobs);

        if knobs.mlock {
            self.qemu_params.push("-overcommit".to_owned());
            self.qemu_params.push("mem-lock=on".to_owned());
        }

        if knobs.stopped {
            self.qemu_params.push("-S".to_owned());
        }

        self
    }

    /// util functions, setup memory-related boolean configurations
    fn add_knobs_memory(&mut self, knobs: &Knobs) {
        if self.memory.size.is_empty() {
            return;
        }
        let dimm_name = "dimm1";
        let mut obj_mem_params = if knobs.hugepages {
            format!(
                "memory-backend-file,id={},size={},mem-path=/dev/hugepages",
                dimm_name, &self.memory.size
            )
        } else if knobs.file_backed_mem && !self.memory.path.is_empty() {
            format!(
                "memory-backend-file,id={},size={},mem_path={}",
                dimm_name, &self.memory.size, &self.memory.path
            )
        } else {
            format!(
                "memory-backend-file,id={},size={}",
                dimm_name, self.memory.size
            )
        };
        let numa_mem_params = format!("node,memdev={}", dimm_name);

        if knobs.mem_shared {
            obj_mem_params += ",share=on";
        }

        if knobs.mem_prealloc {
            obj_mem_params += ",prealloc=on";
        }

        self.qemu_params.push("-object".to_owned());
        self.qemu_params.push(obj_mem_params);

        if self.is_dimm_supported() {
            self.qemu_params.push("-numa".to_owned());
            self.qemu_params.push(numa_mem_params);
        } else {
            self.qemu_params.push("-machine".to_owned());
            self.qemu_params
                .push(format!("memory-backend={}", dimm_name));
        }
    }

    fn is_dimm_supported(&self) -> bool {
        let arch = std::env::consts::ARCH;
        match arch {
            "x86_64" | "powerpc64" | "aarch64" | "x86" => {
                self.machine.machine_type != MACHINE_TYPE_MICROVM
            }

            _ => false,
        }
    }

    pub fn add_io_threads(mut self, io_threads: &[IoThread]) -> Self {
        for thread in io_threads {
            if !thread.id.is_empty() {
                self.qemu_params.push("-object".to_owned());
                self.qemu_params.push(format!("iothread,id={}", &thread.id));
            }
        }
        self
    }

    /// append_fds append a list of file descriptors to the qemu configuration
    /// and returns a slice of offset file descriptors that will be seen by
    /// the qemu process
    pub fn append_fds(&mut self, fds: &[RawFd]) -> Vec<i32> {
        let mut int_fds = vec![];
        let old_length = self.fds.len();
        fds.iter().for_each(|fd| self.fds.push(*fd));

        // 3 is because no stdin, stdout, stderr
        for i in 0..fds.len() {
            int_fds.push(old_length as i32 + i as i32 + 3);
        }
        int_fds
    }

    pub fn add_incoming(mut self, incoming: &Incoming) -> Self {
        let uri = match incoming.migration_type.as_str() {
            MIGRATION_EXEC => {
                format!("exec:{}", incoming.exec)
            }
            MIGRATION_FD => {
                let fds = self.append_fds(&[incoming.fd]);
                format!("fd:{}", fds[0])
            }
            MIGRATION_DEFER => "defer".to_string(),
            _ => {
                return self;
            }
        };
        self.qemu_params.push("-S".to_owned());
        self.qemu_params.push("-incoming".to_owned());
        self.qemu_params.push(uri);
        self
    }

    pub fn add_pflash_param(mut self, pflashs: &[String]) -> Self {
        for pflash in pflashs {
            self.qemu_params.push("-pflash".to_owned());
            self.qemu_params.push(pflash.to_string());
        }
        self
    }

    pub fn add_pid_file(mut self, pid_file: &str) -> Self {
        if !pid_file.is_empty() {
            self.qemu_params.push("-pidfile".to_owned());
            self.qemu_params.push(pid_file.to_owned());
        }
        self
    }

    pub fn add_log_file(mut self, log_file: &str) -> Self {
        if !log_file.is_empty() {
            self.qemu_params.push("-D".to_owned());
            self.qemu_params.push(log_file.to_owned());
        }
        self
    }

    pub fn add_fwcfg(mut self, fw_cfgs: &[FwCfg]) -> Self {
        // todo: qmplogger
        for fwcfg in fw_cfgs {
            if !fwcfg.valid() {
                continue;
            }
            fwcfg.qemu_params(&mut self);
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
            uid: self.uid,
            gid: self.gid,
            groups: self.groups.clone(),
            name: self.name.clone(),
            uuid: self.uuid.clone(),
            cpu_model: self.cpu_model.clone(),
            seccomp_sandbox: self.seccomp_sandbox.clone(),
            machine: self.machine.clone(),
            devices: vec![],
            fds: self.fds.clone(),
            pflashs: self.pflashs.clone(),
            io_threads: self.io_threads.clone(),
            log_file: self.log_file.clone(),
            pid_file: self.pid_file.clone(),
            vga: self.vga.clone(),
            kernel: self.kernel.clone(),
            memory: self.memory.clone(),
            smp: self.smp.clone(),
            no_graphic: self.no_graphic,
            global_params: self.global_params.clone(),
            bios: self.bios.clone(),
            qemu_params: self.qemu_params.clone(),
            rtc: self.rtc.clone(),
            knobs: self.knobs,
            qmp_sockets: self.qmp_sockets.clone(),
            incoming: self.incoming.clone(),
            fw_cfgs: self.fw_cfgs.clone(),
        }
    }
}
