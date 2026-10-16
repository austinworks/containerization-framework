//! Stand-ins for `LocalContentStore.swift`: `LocalContentStore` and
//! `Content`.

use super::CzOutcome;
use std::convert::Infallible;

taken!(
  local_content_store -> CzLocalContentStore,
  content -> CzContent,
  ingest_session_id -> String,
  ingest_session_directory -> String,
);

handles!(CzLocalContentStore, CzContent);

failing!(cz_local_content_store_new(&str), cz_local_content_open(&str));

impl CzLocalContentStore {
  pub(crate) fn get(&self, _digest: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn duplicate(&self) -> CzLocalContentStore {
    match self.0 {}
  }

  pub(crate) fn new_ingest_session(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn complete_ingest_session(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn cancel_ingest_session(&self, _id: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn delete_digests(&self, _digests: Vec<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn delete_keeping(&self, _keeping: Vec<String>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn total_allocated_size(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn ingest(&self, _body: Box<dyn FnOnce(String) -> bool>) -> CzOutcome {
    match self.0 {}
  }
}

impl CzContent {
  pub(crate) fn is_some(&self) -> bool {
    match self.0 {}
  }

  pub(crate) fn path(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn digest(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn size(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn data(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn data_range(&self, _offset: u64, _length: usize) -> CzOutcome {
    match self.0 {}
  }
}
