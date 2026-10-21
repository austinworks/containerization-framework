//! Stand-ins for `Model/ImageModel.swift`: platforms and the OCI image types.

use super::CzOutcome;
use crate::containerization_oci;

taken!(
  platform -> containerization_oci::image::Platform,
  platforms -> Vec<containerization_oci::image::Platform>,
  descriptor -> containerization_oci::image::Descriptor,
  index -> containerization_oci::image::Index,
  manifest -> containerization_oci::image::Manifest,
  oci_image -> containerization_oci::image::Image,
);

failing!(
  cz_platform_current(),
  cz_platform_parse(&str),
  cz_platform_description(containerization_oci::image::Platform),
  cz_platform_equals(
    containerization_oci::image::Platform,
    containerization_oci::image::Platform
  ),
  cz_platform_matches(
    containerization_oci::image::Platform,
    containerization_oci::image::Platform
  ),
  cz_index_runnable_platforms(containerization_oci::image::Index),
);
