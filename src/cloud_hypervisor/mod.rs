//! Containerization's `CloudHypervisor` module, the client for the
//! cloud-hypervisor VMM that Containerization's Linux backend runs. Only
//! the device configurations Containerization's macOS API returns are bound,
//! in [`device`], as Swift declares them in `Types/DeviceConfigs.swift`.

pub mod device;
