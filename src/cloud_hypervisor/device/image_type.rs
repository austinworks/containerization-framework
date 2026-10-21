//! `CloudHypervisor.ImageType`.

raw_values!(
  /// `CloudHypervisor.ImageType`, the on-disk format of a
  /// [`super::DiskConfig`]'s backing file. Without one, cloud-hypervisor
  /// takes the format as unknown and rejects writes to the disk.
  ImageType {
    Raw = "Raw",
    Qcow2 = "Qcow2",
    FixedVhd = "FixedVhd",
    Vhdx = "Vhdx",
    Unknown = "Unknown",
  }
);
