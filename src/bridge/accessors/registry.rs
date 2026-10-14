//! Readers for what registry access and the keychain return.

use crate::bridge::ffi;
use crate::containerization_os::keychain;
use crate::error::Error;
use crate::platform;

impl ffi::CzOutcome {
  /// The `[RegistryInfo]` an outcome holds, its dates crossing as seconds
  /// since 1970.
  pub(crate) fn registry_infos(&self) -> Result<Vec<keychain::RegistryInfo>, Error> {
    self
      .list(|info| {
        Ok(keychain::RegistryInfo {
          hostname: info.registry_info_hostname(),
          username: info.registry_info_username(),
          modified_date: platform::system_time(info.registry_info_modified_date())?,
          created_date: platform::system_time(info.registry_info_created_date())?,
        })
      })
      .into_iter()
      .collect()
  }
}
