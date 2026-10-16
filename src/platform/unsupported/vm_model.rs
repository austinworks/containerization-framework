//! Stand-ins for `Model/VMModel.swift`: an `AttachedFilesystem`, read field by
//! field.

use super::CzOutcome;
use crate::containerization::vm;
use std::collections::BTreeMap;

taken!(
  attached_filesystem -> vm::AttachedFilesystem,
  attached_filesystems_by_id -> BTreeMap<String, Vec<vm::AttachedFilesystem>>,
);
