//! Stand-ins for `IO.swift`: ContainerizationIO's `ReadStream`.

use super::CzOutcome;
use std::convert::Infallible;

taken!(read_stream -> CzReadStream);

handles!(CzReadStream, CzDataStream);

failing!(
  cz_read_stream_new(),
  cz_read_stream_with_url(&str, usize),
  cz_read_stream_with_data(Vec<u8>, usize),
);

impl CzReadStream {
  pub(crate) fn reset(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn data_stream(&self) -> CzDataStream {
    match self.0 {}
  }
}

impl CzDataStream {
  pub(crate) fn next(&self) -> CzOutcome {
    match self.0 {}
  }
}
