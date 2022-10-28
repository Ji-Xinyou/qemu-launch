use qemu_manip::config;
use qemu_manip::qemu::Qemu;

fn main() {
    let config = config::QemuConfig::builder();
    let qemu = Qemu::from_config(config);
    qemu.dump();
    qemu.launch().expect("launched fail");
}
