use super::Filter;
use super::Format;
use super::Options;

/// `ArchiveWriterConfiguration`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveWriterConfiguration {
  pub format: Format,
  pub filter: Filter,
  pub options: Vec<Options>,
  pub locales: Vec<String>,
}

impl ArchiveWriterConfiguration {
  /// `ArchiveWriterConfiguration.defaultLocales`.
  pub const DEFAULT_LOCALES: &[&str] = &["en_US.UTF-8", "C.UTF-8"];

  /// `ArchiveWriterConfiguration(format:filter:options:locales:)`, with
  /// Swift's defaults: no options, and [`Self::DEFAULT_LOCALES`].
  pub fn new(format: Format, filter: Filter) -> Self {
    Self {
      format,
      filter,
      options: Vec::new(),
      locales: Self::default_locales(),
    }
  }

  /// [`Self::DEFAULT_LOCALES`], owned.
  pub(super) fn default_locales() -> Vec<String> {
    Self::DEFAULT_LOCALES
      .iter()
      .map(|locale| locale.to_string())
      .collect()
  }
}
