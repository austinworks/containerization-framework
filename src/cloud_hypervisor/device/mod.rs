//! The device configurations nested in Swift's `CloudHypervisor` namespace.

mod disk_config;
mod image_type;

pub use self::disk_config::DiskConfig;
pub use self::image_type::ImageType;
