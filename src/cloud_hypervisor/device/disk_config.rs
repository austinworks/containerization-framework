//! `CloudHypervisor.DiskConfig`.

use super::ImageType;

/// `CloudHypervisor.DiskConfig`, a virtio-blk disk, which
/// [`crate::containerization::container::Mount::ch_disk_config`] describes a
/// block mount as.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiskConfig {
  /// The path of the disk image file.
  pub path: String,
  pub readonly: Option<bool>,
  /// Whether cloud-hypervisor opens the image with `O_DIRECT`.
  pub direct: Option<bool>,
  pub iommu: Option<bool>,
  pub id: Option<String>,
  pub pci_segment: Option<u16>,
  pub image_type: Option<ImageType>,
}

impl DiskConfig {
  /// `CloudHypervisor.DiskConfig(path:)`, its other arguments at their
  /// default, `nil`.
  pub fn new(path: impl Into<String>) -> Self {
    Self {
      path: path.into(),
      readonly: None,
      direct: None,
      iommu: None,
      id: None,
      pci_segment: None,
      image_type: None,
    }
  }
}
