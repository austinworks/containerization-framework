//! Virtual machines: the kernel they boot and the platform it runs, how they
//! are configured, the manager that creates them, the instances it makes and
//! their state, what is attached to them, and the client of the agent running
//! in them.

mod attached_filesystem;
mod boot_log;
mod filesystem_operation;
pub mod kernel;
pub mod system_platform;
mod virtiofs_layout;
mod virtual_machine_instance_state;
mod vm_configuration;
mod vm_resources;
mod vminitd;
mod vsock_listener;
mod vz_virtual_machine_instance;
pub mod vz_virtual_machine_manager;

pub use self::attached_filesystem::AttachedFilesystem;
pub use self::boot_log::BootLog;
pub use self::filesystem_operation::FilesystemOperation;
pub use self::kernel::Kernel;
pub use self::system_platform::SystemPlatform;
pub use self::virtiofs_layout::VirtiofsLayout;
pub use self::virtual_machine_instance_state::VirtualMachineInstanceState;
pub use self::vm_configuration::VMConfiguration;
pub use self::vm_resources::VMResources;
pub use self::vminitd::Vminitd;
pub use self::vsock_listener::VsockListener;
pub use self::vz_virtual_machine_instance::VZVirtualMachineInstance;
pub use self::vz_virtual_machine_manager::VZVirtualMachineManager;
