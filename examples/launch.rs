use qemu_launch::config;
use qemu_launch::qemu::Qemu;

fn main() {
    let config = config::QemuConfig::builder();
    // todo: fill in the config
    let qemu = Qemu::from_config(config);
    qemu.dump();
    qemu.launch().expect("launched fail");
}
