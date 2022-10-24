use qemu_manip::config;
use qemu_manip::qemu::Qemu;

fn main() {
    let toml_path = "./examples/examples.toml";
    let config = config::QemuConfig::from_toml(toml_path);
    let qemu = Qemu::from_config(config);
    qemu.dump();
    qemu.launch().expect("launched fail");
}
