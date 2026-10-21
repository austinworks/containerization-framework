use super::address;

/// `InterfaceAddress`, the addresses
/// [`crate::containerization::vm::Vminitd::address_add`] gives an interface.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InterfaceAddress {
  pub ipv4_address: address::CIDRv4,
  pub ipv6_address: Option<address::CIDRv6>,
}
