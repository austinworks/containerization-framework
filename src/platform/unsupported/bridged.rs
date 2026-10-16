//! Stand-ins for `Bridged.swift`: the outcome, and the readers of a shape
//! rather than a type.

use std::collections::BTreeMap;

/// An outcome that is always a failure, so nothing is ever taken from it.
pub(crate) struct CzOutcome;

impl CzOutcome {
  pub(crate) fn error(&self) -> Option<String> {
    Some(super::MACOS_ONLY.to_string())
  }
}

taken!(
  is_some -> bool,
  len -> usize,
  boolean -> bool,
  int32 -> i32,
  optional_int32 -> Option<i32>,
  int32_map -> BTreeMap<String, i32>,
  integer -> i64,
  number -> u64,
  uint128 -> u128,
  text -> String,
  optional_text -> Option<String>,
  strings -> Vec<String>,
  has_bytes -> bool,
  bytes -> Vec<u8>,
);
