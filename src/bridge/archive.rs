//! `ContainerizationArchive`'s writer configuration, which Swift reads.

use crate::containerization_archive::ArchiveWriterConfiguration as RustArchiveWriterConfiguration;

use super::ffi::ArchiveOptionKind;

#[swift_bridge::bridge]
mod ffi {
  #[swift_bridge(already_declared)]
  enum ArchiveOptionKind {}

  extern "Rust" {
    // Its enums are their `rawValue`s. An option's level and format are
    // read only for the kinds that have them.
    type RustArchiveWriterConfiguration;
    fn format(self: &RustArchiveWriterConfiguration) -> &str;
    fn filter(self: &RustArchiveWriterConfiguration) -> &str;
    #[swift_bridge(swift_name = "optionsLen")]
    fn options_len(self: &RustArchiveWriterConfiguration) -> usize;
    #[swift_bridge(swift_name = "optionKindAt")]
    fn option_kind_at(self: &RustArchiveWriterConfiguration, index: usize) -> ArchiveOptionKind;
    #[swift_bridge(swift_name = "optionCompressionLevelAt")]
    fn option_compression_level_at(self: &RustArchiveWriterConfiguration, index: usize) -> u32;
    #[swift_bridge(swift_name = "optionXattrFormatAt")]
    fn option_xattr_format_at(self: &RustArchiveWriterConfiguration, index: usize) -> &str;
    #[swift_bridge(swift_name = "localesLen")]
    fn locales_len(self: &RustArchiveWriterConfiguration) -> usize;
    #[swift_bridge(swift_name = "localesAt")]
    fn locales_at(self: &RustArchiveWriterConfiguration, index: usize) -> &str;
  }
}
