//! Containerization's `ContainerizationExtras` module.
//!
//! Each type wraps the Swift type of the same name. The network address types
//! are in [`address`].

pub mod address;
mod default_route;
mod interface_address;
mod link_route;
mod progress_event;
pub mod proxy_utils;

pub use self::default_route::DefaultRoute;
pub use self::interface_address::InterfaceAddress;
pub use self::link_route::LinkRoute;
pub use self::progress_event::ProgressEvent;
pub use self::progress_event::ProgressHandler;
