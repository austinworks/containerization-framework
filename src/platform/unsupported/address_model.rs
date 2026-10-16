//! Stand-ins for `Model/AddressModel.swift`: the address types, read field by
//! field.

use super::CzOutcome;
use crate::containerization_extras::address;

taken!(
  ipv4_address -> address::IPv4Address,
  ipv6_address -> address::IPv6Address,
  ip_address -> address::IPAddress,
  prefix -> address::Prefix,
  cidr_v4 -> address::CIDRv4,
  cidr_v6 -> address::CIDRv6,
  cidr -> address::CIDR,
  mac_address -> address::MACAddress,
  optional_ipv4_address -> Option<address::IPv4Address>,
  optional_ipv6_address -> Option<address::IPv6Address>,
  optional_cidr_v6 -> Option<address::CIDRv6>,
  optional_mac_address -> Option<address::MACAddress>,
);
