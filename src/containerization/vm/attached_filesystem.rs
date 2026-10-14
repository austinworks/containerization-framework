/// `AttachedFilesystem`, a filesystem attached to a VM as its guest mounts
/// it. The initializer that takes an `AddressAllocator` isn't bound.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttachedFilesystem {
  pub r#type: String,
  pub source: String,
  pub destination: String,
  pub options: Vec<String>,
}
