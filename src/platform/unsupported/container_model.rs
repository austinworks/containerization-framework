//! Stand-ins for `Model/ContainerModel.swift`: a `Mount` and an
//! `ExitStatus`, read field by field.

use super::CzOutcome;
use crate::containerization::container;

taken!(
  mount -> container::Mount,
  optional_mount -> Option<container::Mount>,
  exit_code -> i32,
  exited_at -> f64,
);
