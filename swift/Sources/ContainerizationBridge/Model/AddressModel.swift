//===----------------------------------------------------------------------===//
// The address types: Rust's read into ContainerizationExtras' and
// Containerization's, and Swift's read back by Rust field by field. The Rust
// side is `bridge::accessors::addresses`.
//
// IPv4 and MAC addresses cross as their values, and a prefix as its length.
// IPv6 and IP addresses cross as opaque Rust types.
//===----------------------------------------------------------------------===//

import Containerization
import ContainerizationExtras
import Foundation

// MARK: Rust to Swift

extension IPv6Address {
  init(_ address: RustIPv6AddressRef) {
    self.init(UInt128(address.valueHigh()) << 64 | UInt128(address.valueLow()), zone: address.zone()?.toString())
  }
}

extension IPAddress {
  init(_ address: RustIpAddressRef) {
    self = address.holdsV6() ? .v6(IPv6Address(address.v6())) : .v4(IPv4Address(address.v4Value()))
  }
}

extension Prefix {
  /// A length Rust holds, which Swift made.
  init(bridged length: UInt8) throws {
    guard let prefix = Prefix(length: length) else {
      throw BridgeError.malformed("prefix length", String(length))
    }
    self = prefix
  }
}

extension CIDRv4 {
  init(bridged address: UInt32, prefix: UInt8) throws {
    try self.init(IPv4Address(address), prefix: Prefix(bridged: prefix))
  }
}

extension CIDRv6 {
  init(bridged address: RustIPv6AddressRef, prefix: UInt8) throws {
    try self.init(IPv6Address(address), prefix: Prefix(bridged: prefix))
  }
}

extension CIDR {
  /// The case Rust holds, made directly as Rust's was.
  init(bridged address: RustIpAddressRef, prefix: UInt8) throws {
    let prefix = try Prefix(bridged: prefix)
    self =
      switch IPAddress(address) {
      case .v4(let address): .v4(address, prefix)
      case .v6(let address): .v6(address, prefix)
      }
  }
}

extension InterfaceAddress {
  init(_ address: RustInterfaceAddressRef) throws {
    self.init(
      ipv4Address: try CIDRv4(bridged: address.ipv4AddressValue(), prefix: address.ipv4AddressPrefix()),
      ipv6Address: address.hasIpv6Address()
        ? try CIDRv6(bridged: address.ipv6AddressAddress(), prefix: address.ipv6AddressPrefix())
        : nil
    )
  }
}

extension LinkRoute {
  init(_ route: RustLinkRouteRef) {
    self.init(
      ipv4Destination: route.ipv4Destination().map { IPv4Address($0) },
      ipv4Source: route.ipv4Source().map { IPv4Address($0) },
      ipv6Destination: route.hasIpv6Destination() ? IPv6Address(route.ipv6Destination()) : nil,
      ipv6Source: route.hasIpv6Source() ? IPv6Address(route.ipv6Source()) : nil
    )
  }
}

extension DefaultRoute {
  init(_ route: RustDefaultRouteRef) {
    self.init(
      ipv4Gateway: route.ipv4Gateway().map { IPv4Address($0) },
      ipv6Gateway: route.hasIpv6Gateway() ? IPv6Address(route.ipv6Gateway()) : nil
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  /// Whether the address held, alone or in a CIDR block, is IPv6.
  func isIPv6() -> Bool {
    heldAddress() is IPv6Address
  }

  func ipv4Value() -> UInt32 {
    (cast(heldAddress()) as IPv4Address).value
  }

  func ipv6High() -> UInt64 {
    UInt64((cast(heldAddress()) as IPv6Address).value >> 64)
  }

  func ipv6Low() -> UInt64 {
    UInt64(truncatingIfNeeded: (cast(heldAddress()) as IPv6Address).value)
  }

  func ipv6Zone() -> String? {
    (cast(heldAddress()) as IPv6Address).zone
  }

  func prefixLength() -> UInt8 {
    switch value {
    case let cidr as CIDRv4: cidr.prefix.length
    case let cidr as CIDRv6: cidr.prefix.length
    case let cidr as CIDR: cidr.prefix.length
    default: (taken() as Prefix).length
    }
  }

  func macValue() -> UInt64 {
    (taken() as MACAddress).value
  }

  /// The `IPv4Address` or `IPv6Address` held, alone or in an `IPAddress`, a
  /// CIDR block or a `CIDR`.
  private func heldAddress() -> Any? {
    switch value {
    case let cidr as CIDRv4: cidr.address
    case let cidr as CIDRv6: cidr.address
    case let cidr as CIDR: unwrapped(cidr.address)
    case let address as IPAddress: unwrapped(address)
    default: value
    }
  }

  private func unwrapped(_ address: IPAddress) -> Any {
    switch address {
    case .v4(let address): address
    case .v6(let address): address
    }
  }
}
