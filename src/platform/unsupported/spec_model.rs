//! Stand-ins for `Model/SpecModel.swift` and `Model/Spec/`: the OCI runtime
//! spec's types.

use super::CzOutcome;
use crate::containerization_oci;

taken!(
  spec -> containerization_oci::runtime::Spec,
  process -> containerization_oci::runtime::Process,
  oci_linux_capabilities -> containerization_oci::runtime::LinuxCapabilities,
  posix_rlimit -> containerization_oci::runtime::POSIXRlimit,
  seccomp -> containerization_oci::runtime::LinuxSeccomp,
  runtime_spec_version -> containerization_oci::runtime::RuntimeSpecVersion,
);
