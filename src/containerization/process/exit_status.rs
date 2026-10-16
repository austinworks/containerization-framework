use crate::error::Error;
use crate::platform;
use crate::platform::ffi;
use std::time::SystemTime;

/// `ExitStatus`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExitStatus {
  pub exit_code: i32,
  pub exited_at: SystemTime,
}

impl ExitStatus {
  /// `ExitStatus(exitCode:)`, which Swift stamps with the current time.
  pub fn new(exit_code: i32) -> Result<Self, Error> {
    platform::outcome(ffi::cz_exit_status_new(exit_code), "make an exit status")
      .and_then(|outcome| Self::from_outcome(&outcome))
  }

  /// The `ExitStatus` an outcome holds, whose `exitedAt` crosses as seconds
  /// since 1970.
  pub(crate) fn from_outcome(outcome: &ffi::CzOutcome) -> Result<Self, Error> {
    Ok(Self {
      exit_code: outcome.exit_code(),
      exited_at: platform::system_time(outcome.exited_at())?,
    })
  }
}
