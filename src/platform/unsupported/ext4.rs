//! Stand-ins for `Ext4.swift`: `EXT4.EXT4Reader`, `EXT4.Formatter` and the
//! values they read and write.

use super::CzArchiveReader;
use super::CzImage;
use super::CzOutcome;
use crate::containerization::image;
use crate::containerization_ext4::ext4;
use crate::containerization_oci;
use crate::platform;
use std::convert::Infallible;

taken!(
  ext4_reader -> CzExt4Reader,
  ext4_formatter -> CzExt4Formatter,
  scanned_size -> i64,
  scanned_items -> isize,
  inode_number -> u32,
  inode -> ext4::Inode,
  compressed_name_id -> u8,
  compressed_name_str -> String,
);

handles!(CzExt4Reader, CzExt4Formatter);

failing!(
  cz_ext4_reader_new(&str),
  cz_ext4_reader_read_inline_extended_attributes(Vec<u8>),
  cz_ext4_reader_read_block_extended_attributes(Vec<u8>),
  cz_ext4_file_xattrs_state_read(Vec<u8>, usize, usize),
  cz_ext4_inode_root(),
  cz_ext4_extended_attribute_compress_name(&str),
  cz_ext4_extended_attribute_decompress_name(isize, &str),
  cz_ext4_formatter_new(&str, ext4::formatter::FormatterOptions),
  cz_ext4_formatter_scan_archive_headers(&str, &str, &str),
);

pub(crate) fn cz_ext4_unpack(
  _unpacker: image::EXT4Unpacker,
  image: CzImage,
  _platform: containerization_oci::image::Platform,
  _at: &str,
  _progress: platform::Progress,
) -> CzOutcome {
  match image.0 {}
}

impl CzExt4Reader {
  pub(crate) fn super_block(&self) -> Vec<u8> {
    match self.0 {}
  }

  pub(crate) fn exists(&self, _path: &str, _follow_symlinks: bool) -> bool {
    match self.0 {}
  }

  pub(crate) fn stat(&self, _path: &str, _follow_symlinks: bool) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn list_directory(&self, _path: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn read_file(&self, _at: &str, _offset: u64, _count: Option<usize>, _follow_symlinks: bool) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn export(&self, _archive: &str) -> CzOutcome {
    match self.0 {}
  }
}

impl CzExt4Formatter {
  pub(crate) fn link(&self, _link: &str, _target: &str) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn unlink(&self, _path: &str, _directory_whiteout: bool) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn create(
    &self,
    _path: &str,
    _link: Option<String>,
    _mode: u16,
    _access: f64,
    _modification: f64,
    _creation: f64,
    _now: f64,
    _has_buf: bool,
    _buf: Vec<u8>,
    _uid: Option<u32>,
    _gid: Option<u32>,
    _has_xattrs: bool,
    _xattr_names: Vec<String>,
    _xattr_lengths: Vec<u64>,
    _xattr_values: Vec<u8>,
    _recursion: bool,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn close(&self) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn unpack(
    &self,
    _source: &str,
    _format: &str,
    _compression: &str,
    _progress: platform::Progress,
  ) -> CzOutcome {
    match self.0 {}
  }

  pub(crate) fn unpack_reader(&self, _reader: CzArchiveReader, _progress: platform::Progress) -> CzOutcome {
    match self.0 {}
  }
}
