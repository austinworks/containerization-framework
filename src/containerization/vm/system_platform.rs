//! `SystemPlatform`, and its nested `SystemPlatform.OS` and
//! `SystemPlatform.Architecture`.

use crate::containerization_oci;
use crate::error::Error;
use crate::platform;
use crate::platform::ffi;

raw_values!(
  /// `SystemPlatform.OS`.
  CaseIterable OS { Linux = "linux", Darwin = "darwin" }
);

raw_values!(
  /// `SystemPlatform.Architecture`.
  CaseIterable Architecture { Arm64 = "arm64", Amd64 = "amd64" }
);

/// `SystemPlatform`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SystemPlatform {
  pub os: OS,
  pub architecture: Architecture,
}

impl SystemPlatform {
  /// `SystemPlatform.linuxArm`.
  pub const LINUX_ARM: Self = Self {
    os: OS::Linux,
    architecture: Architecture::Arm64,
  };

  /// `SystemPlatform.linuxAmd`.
  pub const LINUX_AMD: Self = Self {
    os: OS::Linux,
    architecture: Architecture::Amd64,
  };

  /// `SystemPlatform.ociPlatform()`.
  pub fn oci_platform(&self) -> Result<containerization_oci::image::Platform, Error> {
    platform::outcome(
      ffi::cz_system_platform_oci_platform(*self),
      "convert a system platform to OCI's",
    )
    .map(|outcome| outcome.platform())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reads_back_every_raw_value_it_writes() {
    for os in OS::ALL_CASES {
      assert_eq!(OS::from_raw_value(os.raw_value()), Some(*os));
    }
    for architecture in Architecture::ALL_CASES {
      assert_eq!(
        Architecture::from_raw_value(architecture.raw_value()),
        Some(*architecture)
      );
    }
    assert_eq!(OS::from_raw_value("Linux"), None, "raw values are case sensitive");
  }

  #[test]
  fn parses_and_lends_its_raw_value_through_std_traits() {
    assert_eq!("arm64".parse::<Architecture>(), Ok(Architecture::Arm64));
    assert_eq!(OS::Darwin.as_ref(), "darwin");

    let error = "windows".parse::<OS>().unwrap_err();
    assert_eq!(error.action(), "parse a raw value of OS");
  }
}
