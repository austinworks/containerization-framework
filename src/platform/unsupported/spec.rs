//! Stand-ins for `Spec.swift`: what the OCI runtime spec's types compute,
//! seccomp profiles and bundles.

use super::CzOutcome;
use crate::containerization::process;
use crate::containerization::vm;
use crate::containerization_oci;

failing!(
  cz_process_from_image_config(containerization_oci::image::ImageConfig),
  cz_process_description(containerization_oci::runtime::Process),
  cz_hook_description(containerization_oci::runtime::Hook),
  cz_linux_seccomp_decode(Vec<u8>),
  cz_linux_seccomp_default_profile(bool, containerization_oci::runtime::LinuxCapabilities, &str),
  cz_arch_current(),
  cz_arch_current_verified(),
  cz_runtime_spec_version_current(),
  cz_linux_rlimit_to_oci(process::LinuxRLimit),
  cz_linux_capabilities_to_oci(process::LinuxCapabilities),
  cz_system_platform_oci_platform(vm::SystemPlatform),
  cz_bundle_create(&str, containerization_oci::runtime::Spec),
  cz_bundle_create_from_data(&str, Vec<u8>),
  cz_bundle_load(&str),
  cz_bundle_config_path(&str),
  cz_bundle_rootfs_path(&str),
  cz_bundle_delete(&str),
  cz_bundle_load_config(&str),
);
