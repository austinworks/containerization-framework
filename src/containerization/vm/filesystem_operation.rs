use crate::platform::ffi;

/// `FilesystemOperation`, which
/// [`crate::containerization::container::LinuxContainer::filesystem_operation`]
/// performs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FilesystemOperation {
  Freeze,
  Thaw,
  Trim,
}

impl From<FilesystemOperation> for ffi::FilesystemOperationKind {
  fn from(operation: FilesystemOperation) -> Self {
    match operation {
      FilesystemOperation::Freeze => Self::Freeze,
      FilesystemOperation::Thaw => Self::Thaw,
      FilesystemOperation::Trim => Self::Trim,
    }
  }
}
