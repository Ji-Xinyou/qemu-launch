use anyhow::Result;
use log::info;

use crate::config::QemuConfig;

use std::process::Command;

/// the delimiter between parameters
const QEMU_PARAM_DELIMITER: &str = " ";

/// qemu instance information
pub struct Qemu {
    bin_path: String,

    args: Vec<String>,
}

impl Qemu {
    /// new qemu instance
    pub fn new(bin_path: String, args: Vec<String>) -> Self {
        Self { bin_path, args }
    }

    pub fn from_config(config: QemuConfig) -> Self {
        let config = config.build_all();

        Self {
            bin_path: config.bin_path,
            args: config.qemu_params,
        }
    }

    /// launch qemu process with expected parameters
    pub fn launch(&self) -> Result<()> {
        Command::new(&self.bin_path)
            .args(&self.args)
            .spawn()
            .expect("Failed to spawn QEMU process");
        Ok(())
    }
}

// utils
impl Qemu {
    pub fn dump(&self) {
        info!("Binary path: {}\nargs: {:?}", self.bin_path, self.args);
    }
}
