//===----------------------------------------------------------------------===//
// Rust's network values read into Containerization's: NAT interfaces, DNS and
// hosts. The Rust side is `bridge::network` and
// `bridge::accessors::container::network`.
//===----------------------------------------------------------------------===//

import Containerization
import ContainerizationExtras
import Foundation

// MARK: Rust to Swift

extension NATInterface {
  init(_ interface: RustNatInterfaceRef) throws {
    self.init(
      ipv4Address: try CIDRv4(bridged: interface.ipv4AddressValue(), prefix: interface.ipv4Prefix()),
      ipv4Gateway: interface.ipv4Gateway().map { IPv4Address($0) },
      ipv6Address: interface.hasIpv6Address()
        ? try CIDRv6(bridged: interface.ipv6Address(), prefix: interface.ipv6Prefix()) : nil,
      ipv6Gateway: interface.hasIpv6Gateway() ? IPv6Address(interface.ipv6Gateway()) : nil,
      macAddress: interface.macAddress().map { MACAddress($0) },
      mtu: interface.mtu()
    )
  }
}

extension DNS {
  init(_ dns: RustDnsRef) {
    self.init(
      nameservers: strings(dns.nameserversLen(), dns.nameserversAt),
      domain: dns.domain()?.toString(),
      searchDomains: strings(dns.searchDomainsLen(), dns.searchDomainsAt),
      options: strings(dns.optionsLen(), dns.optionsAt)
    )
  }
}

extension Hosts.Entry {
  init(_ entry: RustHostsEntryRef) {
    self.init(
      ipAddress: entry.ipAddress().toString(),
      hostnames: strings(entry.hostnamesLen(), entry.hostnamesAt),
      comment: entry.comment()?.toString()
    )
  }
}

extension Hosts {
  init(_ hosts: RustHostsRef) {
    self.init(
      entries: list(hosts.entriesLen(), hosts.entriesAt).map(Hosts.Entry.init),
      comment: hosts.comment()?.toString()
    )
  }
}

/// The interfaces Rust lends by index, each as the conforming type it is.
func lentInterfaces(
  _ count: UInt,
  _ kind: (UInt) -> InterfaceKind,
  _ nat: (UInt) -> RustNatInterfaceRef,
  _ vmnet: (UInt) -> CzVmnetInterface
) throws -> [any Interface] {
  try list(count) { index -> any Interface in
    switch kind(index) {
    case .Nat: try NATInterface(nat(index))
    case .Vmnet: vmnet(index).interface
    }
  }
}
