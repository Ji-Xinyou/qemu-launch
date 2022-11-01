pub type ObjectTypeRef<'a> = &'a str;
pub type ObjectType = String;

// MemoryBackendFile represents a guest memory mapped file.
pub const MEMORYBACKENDFILE: ObjectTypeRef = "memory-backend-file";
// MemoryBackendEPC represents a guest memory backend EPC for SGX.
pub const MEMORYBACKENDEPC: ObjectTypeRef = "memory-backend-epc";
// TDXGuest represents a TDX object
pub const TDXGUEST: ObjectTypeRef = "tdx-guest";
// SEVGuest represents an SEV guest object
pub const SEVGUEST: ObjectTypeRef = "sev-guest";
// SNPGuest represents an SNP guest object
pub const SNPGUEST: ObjectTypeRef = "sev-snp-guest";
// SecExecGuest represents an s390x Secure Execution (Protected Virtualization in QEMU) object
pub const SECEXECGUEST: ObjectTypeRef = "s390-pv-guest";
// PEFGuest represent ppc64le PEF(Protected Execution Facility) object.
pub const PEFGUEST: ObjectTypeRef = "pef-guest";

pub type DeviceDriverRef<'a> = &'a str;
pub type DeviceDriver = String;

// LegacySerial is the legacy serial device driver
pub const LEGACYSERIAL: DeviceDriverRef = "serial";
// NVDIMM is the Non Volatile DIMM device driver.
pub const NVDIMM: DeviceDriverRef = "nvdimm";
// VirtioNet is the virtio networking device driver.
pub const VIRTIONET: DeviceDriverRef = "virtio-net";
// VirtioNetPCI is the virt-io pci networking device driver.
pub const VIRTIONETPCI: DeviceDriverRef = "virtio-net-pci";
// VirtioNetCCW is the virt-io ccw networking device driver.
pub const VIRTIONETCCW: DeviceDriverRef = "virtio-net-ccw";
// VirtioBlock is the block device driver.
pub const VIRTIOBLOCK: DeviceDriverRef = "virtio-blk";
// Console is the console device driver.
pub const CONSOLE: DeviceDriverRef = "virtconsole";
// Virtio9P is the 9pfs device driver.
pub const VIRTIO9P: DeviceDriverRef = "virtio-9p";
// VirtioSerial is the serial device driver.
pub const VIRTIOSERIAL: DeviceDriverRef = "virtio-serial";
// VirtioSerialPort is the serial port device driver.
pub const VIRTIOSERIALPORT: DeviceDriverRef = "virtserialport";
// VirtioRng is the paravirtualized RNG device driver.
pub const VIRTIORNG: DeviceDriverRef = "virtio-rng";
// VirtioBalloon is the memory balloon device driver.
pub const VIRTIOBALLOON: DeviceDriverRef = "virtio-balloon";
//VhostUserSCSI represents a SCSI vhostuser device type.
pub const VHOSTUSERSCSI: DeviceDriverRef = "vhost-user-scsi";
//VhostUserNet represents a net vhostuser device type.
pub const VHOSTUSERNET: DeviceDriverRef = "virtio-net";
//VhostUserBlk represents a block vhostuser device type.
pub const VHOSTUSERBLK: DeviceDriverRef = "vhost-user-blk";
//VhostUserFS represents a virtio-fs vhostuser device type
pub const VHOSTUSERFS: DeviceDriverRef = "vhost-user-fs";
// PCIBridgeDriver represents a PCI bridge device type.
pub const PCIBRIDGEDRIVER: DeviceDriverRef = "pci-bridge";
// PCIePCIBridgeDriver represents a PCIe to PCI bridge device type.
pub const PCIEPCIBRIDGEDRIVER: DeviceDriverRef = "pcie-pci-bridge";
// VfioPCI is the vfio driver with PCI transport.
pub const VFIOPCI: DeviceDriverRef = "vfio-pci";
// VfioCCW is the vfio driver with CCW transport.
pub const VFIOCCW: DeviceDriverRef = "vfio-ccw";
// VfioAP is the vfio driver with AP transport.
pub const VFIOAP: DeviceDriverRef = "vfio-ap";
// VHostVSockPCI is a generic Vsock vhost device with PCI transport.
pub const VHOSTVSOCKPCI: DeviceDriverRef = "vhost-vsock-pci";
// PCIeRootPort is a PCIe Root Port, the PCIe device should be hotplugged to this port.
pub const PCIEROOTPORT: DeviceDriverRef = "pcie-root-port";
// Loader is the Loader device driver.
pub const LOADER: DeviceDriverRef = "loader";
// SpaprTPMProxy is used for enabling guest to run in secure mode on ppc64le.
pub const SPAPRTPMPROXY: DeviceDriverRef = "spapr-tpm-proxy";

pub type FsDriverRef<'a> = &'a str;
pub type FsDriver = String;

pub const LOCAL: FsDriverRef = "local";
pub const HANDLE: FsDriverRef = "handle";
pub const PROXY: FsDriverRef = "proxy";

pub type SecurityModelRef<'a> = &'a str;
pub type SecurityModel = String;

// None is like passthrough without failure reports.
pub const NONE: SecurityModelRef = "none";
// PassThrough uses the same credentials on both the host and guest.
pub const PASSTHROUGH: SecurityModelRef = "passthrough";
// MappedXattr stores some files attributes as extended attributes.
pub const MAPPEDXATTR: SecurityModelRef = "mapped-xattr";
// MappedFile stores some files attributes in the .virtfs directory.
pub const MAPPEDFILE: SecurityModelRef = "mapped-file";
