//! `ContainerizationEXT4`'s formatter options, which Swift reads.

use crate::containerization_ext4::ext4::formatter::FormatterOptions as RustFormatterOptions;

use super::ffi::JournalModeKind;

#[swift_bridge::bridge]
mod ffi {
  #[swift_bridge(already_declared)]
  enum JournalModeKind {}

  extern "Rust" {
    type RustFormatterOptions;
    #[swift_bridge(swift_name = "blockSize")]
    fn block_size(self: &RustFormatterOptions) -> u32;
    #[swift_bridge(swift_name = "minDiskSize")]
    fn min_disk_size(self: &RustFormatterOptions) -> u64;
    #[swift_bridge(swift_name = "hasJournal")]
    fn has_journal(self: &RustFormatterOptions) -> bool;
    #[swift_bridge(swift_name = "journalSize")]
    fn journal_size(self: &RustFormatterOptions) -> Option<u64>;
    #[swift_bridge(swift_name = "hasJournalMode")]
    fn has_journal_mode(self: &RustFormatterOptions) -> bool;
    #[swift_bridge(swift_name = "journalMode")]
    fn journal_mode(self: &RustFormatterOptions) -> JournalModeKind;
  }
}
