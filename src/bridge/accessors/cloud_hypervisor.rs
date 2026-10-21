//! Readers for CloudHypervisor's `DiskConfig`.

use crate::bridge::ffi;
use crate::cloud_hypervisor::device;

impl ffi::CzOutcome {
  /// The `CloudHypervisor.DiskConfig` an outcome holds, read field by field.
  pub(crate) fn disk_config(&self) -> device::DiskConfig {
    device::DiskConfig {
      path: self.disk_config_path(),
      readonly: self.disk_config_readonly(),
      direct: self.disk_config_direct(),
      iommu: self.disk_config_iommu(),
      id: self.disk_config_id(),
      pci_segment: self.disk_config_pci_segment(),
      image_type: self
        .disk_config_image_type()
        .map(|raw_value| device::ImageType::from_swift(&raw_value)),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::bridge::ffi;
  use crate::cloud_hypervisor::device;

  #[test]
  fn copies_swifts_image_type_raw_values() {
    assert_eq!(
      ffi::cz_image_type_raw_values(),
      device::ImageType::ALL
        .iter()
        .map(|image_type| image_type.raw_value())
        .collect::<Vec<_>>()
    );
  }
}
