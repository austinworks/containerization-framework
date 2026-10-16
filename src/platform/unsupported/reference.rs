//! Stand-ins for `Reference.swift`: `Reference` and `ParsedDigest`.

use super::CzOutcome;
use std::convert::Infallible;

taken!(
  reference -> CzReference,
  parsed_digest_encoded -> String,
);

handles!(CzReference);

failing!(
  cz_reference_new(&str, Option<String>, Option<String>, Option<String>),
  cz_reference_parse(&str),
  cz_reference_with_name(&str),
  cz_reference_resolve_domain(&str),
  cz_parsed_digest_parse(&str),
  cz_parsed_digest_parse_path_component(&str),
  cz_parsed_digest_is_valid(&str),
  cz_parsed_digest_description(&str),
  cz_parsed_digest_path(&str, &str),
);

impl CzReference {
  pub(crate) fn domain(&self) -> Option<String> {
    match self.0 {}
  }

  pub(crate) fn resolved_domain(&self) -> Option<String> {
    match self.0 {}
  }

  pub(crate) fn path(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn tag(&self) -> Option<String> {
    match self.0 {}
  }

  pub(crate) fn digest(&self) -> Option<String> {
    match self.0 {}
  }

  pub(crate) fn name(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn description(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn with_tag(&self, _tag: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn with_digest(&self, _digest: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn normalize(&self) {
    match self.0 {}
  }
}
