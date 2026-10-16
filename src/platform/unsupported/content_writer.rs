//! Stand-ins for `ContentWriter.swift`.

use super::CzOutcome;
use std::convert::Infallible;

taken!(
  content_writer -> CzContentWriter,
  written_size -> i64,
  written_digest -> String,
);

handles!(CzContentWriter);

failing!(cz_content_writer_new(&str), cz_content_writer_copy(&str, &str));

impl CzContentWriter {
  pub(crate) fn write(&self, _data: Vec<u8>) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create(&self, _from: &str) -> CzOutcome {
    match self.0 {}
  }
}
