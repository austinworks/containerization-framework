use crate::containerization::GIB;
use crate::containerization::MIB;

/// `VMResources`, the size of the VM that containers run in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VMResources {
  pub cpus: u32,
  pub memory_in_bytes: u64,
}

impl VMResources {
  /// `VMResources.guestMemoryOverhead`. Swift never adds it, so a caller who
  /// wants the headroom adds it to `memory_in_bytes`.
  pub const GUEST_MEMORY_OVERHEAD: u64 = 128 * MIB;
}

/// `VMResources.default`.
impl Default for VMResources {
  fn default() -> Self {
    Self {
      cpus: 4,
      memory_in_bytes: GIB,
    }
  }
}
