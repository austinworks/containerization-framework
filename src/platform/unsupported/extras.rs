//! Stand-ins for `Extras.swift`: `ProgressEvent` and `ProxyUtils`.

use super::CzOutcome;
use super::ProgressKind;

failing!(
  cz_progress_event_event(ProgressKind, i64),
  cz_proxy_from_environment(Option<String>, &str, bool, Vec<String>, Vec<String>),
);
