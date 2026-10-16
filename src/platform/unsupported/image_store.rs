//! Stand-ins for `ImageStore.swift`: `ImageStore`, `Image`, `InitImage` and
//! `KernelImage`.

use super::CzAuthentication;
use super::CzLocalContentStore;
use super::CzOutcome;
use crate::containerization::image;
use crate::containerization::vm;
use crate::containerization_oci;
use crate::platform;
use std::convert::Infallible;

taken!(
  image_store -> CzImageStore,
  image -> CzImage,
  images -> CzImages,
  init_image -> CzInitImage,
  kernel_image -> CzKernelImage,
  kernel -> vm::Kernel,
);

handles!(CzImageStore, CzImages, CzImage, CzInitImage, CzKernelImage);

failing!(cz_image_store_new(&str), cz_image_store_default());

/// `ImageStore(path:contentStore:)` and `Image(description:contentStore:)`.
impl CzLocalContentStore {
  pub(crate) fn image_store(&self, _path: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn image(&self, _description: image::Description) -> CzImage {
    match self.0 {}
  }
}

impl CzImageStore {
  pub(crate) fn path(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn create_init_image(
    &self,
    _reference: &str,
    _rootfs: &str,
    _platform: containerization_oci::image::Platform,
    _label_keys: Vec<String>,
    _label_values: Vec<String>,
    _content_store: CzLocalContentStore,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create_kernel_image(
    &self,
    _reference: &str,
    _binaries: Vec<vm::Kernel>,
    _label_keys: Vec<String>,
    _label_values: Vec<String>,
    _content_store: CzLocalContentStore,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn get(&self, _reference: &str, _pull: bool) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn list(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn delete(&self, _reference: &str, _perform_cleanup: bool) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn tag(&self, _existing: &str, _new: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn pull(
    &self,
    _reference: &str,
    _has_platform: bool,
    _platform: containerization_oci::image::Platform,
    _insecure: bool,
    _auth: CzAuthentication,
    _progress: platform::Progress,
    _max_concurrent_downloads: usize,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn push(
    &self,
    _reference: &str,
    _has_platform: bool,
    _platform: containerization_oci::image::Platform,
    _insecure: bool,
    _auth: CzAuthentication,
    _progress: platform::Progress,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn push_all(
    &self,
    _references: Vec<String>,
    _has_platform: bool,
    _platform: containerization_oci::image::Platform,
    _insecure: bool,
    _auth: CzAuthentication,
    _max_concurrent_uploads: usize,
    _progress: platform::Progress,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn get_init_image(
    &self,
    _reference: &str,
    _auth: CzAuthentication,
    _progress: platform::Progress,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn save(
    &self,
    _references: Vec<String>,
    _out: &str,
    _has_platform: bool,
    _platform: containerization_oci::image::Platform,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn clean_up_orphaned_blobs(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn calculate_orphaned_blobs_size(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create(&self, _description: image::Description) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn load(&self, _from: &str, _progress: platform::Progress) -> CzOutcome {
    match self.0 {}
  }
}

impl CzImages {
  pub(crate) fn len(&self) -> usize {
    match self.0 {}
  }

  pub(crate) fn at(&self, _index: usize) -> CzImage {
    match self.0 {}
  }
}

impl CzImage {
  pub(crate) fn duplicate(&self) -> CzImage {
    match self.0 {}
  }

  pub(crate) fn init_image(&self) -> CzInitImage {
    match self.0 {}
  }

  pub(crate) fn kernel_image(&self) -> CzKernelImage {
    match self.0 {}
  }

  pub(crate) fn reference(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn digest(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn media_type(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn descriptor(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn index(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn manifest(&self, _platform: containerization_oci::image::Platform) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn descriptor_for(&self, _platform: containerization_oci::image::Platform) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn config(&self, _platform: containerization_oci::image::Platform) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn referenced_digests(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn get_content(&self, _digest: &str) -> CzOutcome {
    match self.0 {}
  }
}

impl CzKernelImage {
  pub(crate) fn name(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn kernel(&self, _platform: vm::SystemPlatform) -> CzOutcome {
    match self.0 {}
  }
}

impl CzInitImage {
  pub(crate) fn name(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn init_block(&self, _at: &str, _platform: vm::SystemPlatform) -> CzOutcome {
    match self.0 {}
  }
}
