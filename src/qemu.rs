use anyhow::{anyhow, Context, Result};

use crate::config::QemuConfig;

use std::process::Command;

/// the delimiter between parameters
const QEMU_PARAM_DELIMITER: &str = " ";

/// qemu instance information
pub struct Qemu {
    bin_path: String,

    params: String,
}

impl Qemu {
    /// new qemu instance
    pub fn new(bin_path: String, params: Vec<&str>) -> Self {
        let params = params.join(QEMU_PARAM_DELIMITER);
        Self { bin_path, params }
    }

    pub fn from_config(config: QemuConfig) -> Self {
        unimplemented!()
    }

    /// launch qemu process with expected parameters
    pub fn launch(&self) -> Result<()> {
        Command::new(&self.bin_path)
            .arg(&self.params)
            .spawn()
            .expect("Failed to spawn QEMU process");
        Ok(())
    }
}
