//===----------------------------------------------------------------------===//
// ContainerizationOS: `Terminal`, `CapabilityName`, `CapabilitySet`,
// `KeychainQuery`, `Sysctl`, `ProcNetSnmp` and `File`.
//
// `CapabilityName` and `CapabilitySet` cross as their `description`s.
//===----------------------------------------------------------------------===//

import ContainerizationOS
import Foundation

/// For a unit test: `CapabilityName.allCases`' descriptions.
func capabilityNameDescriptions() -> RustVec<RustString> {
  rustStrings(CapabilityName.allCases.map(\.description))
}

/// For a unit test: `CapabilityName.allCases`' `capValue`s.
func capabilityNameCapValues() -> RustVec<UInt32> {
  rustVec(CapabilityName.allCases.map(\.capValue))
}

/// For a unit test: each `CapabilitySet`'s description, in the order of Rust's
/// `ALL`.
func capabilitySetDescriptions() -> RustVec<RustString> {
  rustStrings(
    [ContainerizationOS.CapabilitySet.bounding, .effective, .inheritable, .permitted, .ambient].map(\.description))
}

func parseCapabilityName(rawValue: RustStr) -> CzOutcome {
  let rawValue = rawValue.toString()

  return CzOutcome { try CapabilityName(rawValue: rawValue).description }
}

func parseCapabilitySet(rawValue: RustStr) -> CzOutcome {
  let rawValue = rawValue.toString()

  // Qualified: the bridge has a `CapabilitySet` of its own.
  return CzOutcome { try ContainerizationOS.CapabilitySet(rawValue: rawValue).description }
}

func newTerminal(descriptor: Int32, setInitState: Bool) -> CzOutcome {
  CzOutcome { CzTerminal(try Terminal(descriptor: descriptor, setInitState: setInitState)) }
}

func currentTerminal() -> CzOutcome {
  CzOutcome { CzTerminal(try Terminal.current) }
}

func createTerminal(hasInitialSize: Bool, width: UInt16, height: UInt16) -> CzOutcome {
  let initialSize = hasInitialSize ? Terminal.Size(width: width, height: height) : nil

  return CzOutcome { try Terminal.create(initialSize: initialSize) }
}

/// A `Terminal`.
final class CzTerminal: Sendable {
  let terminal: Terminal

  init(_ terminal: Terminal) {
    self.terminal = terminal
  }

  /// A copy of the terminal, for a call to take.
  func duplicate() -> CzTerminal {
    CzTerminal(terminal)
  }

  func handle() -> Int32 {
    terminal.handle.fileDescriptor
  }

  func write(data: RustVec<UInt8>) -> CzOutcome {
    let data = Data(data)

    return CzOutcome { try terminal.write(data) }
  }

  func size() -> CzOutcome {
    CzOutcome { try terminal.size }
  }

  func resizeFrom(pty: CzTerminal) -> CzOutcome {
    CzOutcome { try terminal.resize(from: pty.terminal) }
  }

  func resizeSize(width: UInt16, height: UInt16) -> CzOutcome {
    CzOutcome { try terminal.resize(size: Terminal.Size(width: width, height: height)) }
  }

  func resize(width: UInt16, height: UInt16) -> CzOutcome {
    CzOutcome { try terminal.resize(width: width, height: height) }
  }

  func setraw() -> CzOutcome {
    CzOutcome { try terminal.setraw() }
  }

  func enableEcho() -> CzOutcome {
    CzOutcome { try terminal.enableEcho() }
  }

  func disableEcho() -> CzOutcome {
    CzOutcome { try terminal.disableEcho() }
  }

  func close() -> CzOutcome {
    CzOutcome { try terminal.close() }
  }

  func reset() -> CzOutcome {
    CzOutcome { try terminal.reset() }
  }

  func tryReset() {
    terminal.tryReset()
  }
}

func keychainQuerySave(
  securityDomain: RustStr,
  accessGroup: RustString?,
  hostname: RustStr,
  username: RustStr,
  password: RustStr
) -> CzOutcome {
  let securityDomain = securityDomain.toString()
  let accessGroup = accessGroup?.toString()
  let hostname = hostname.toString()
  let username = username.toString()
  let password = password.toString()

  return CzOutcome {
    try KeychainQuery().save(
      securityDomain: securityDomain,
      accessGroup: accessGroup,
      hostname: hostname,
      username: username,
      password: password
    )
  }
}

func keychainQueryDelete(securityDomain: RustStr, accessGroup: RustString?, hostname: RustStr) -> CzOutcome {
  let securityDomain = securityDomain.toString()
  let accessGroup = accessGroup?.toString()
  let hostname = hostname.toString()

  return CzOutcome {
    try KeychainQuery().delete(securityDomain: securityDomain, accessGroup: accessGroup, hostname: hostname)
  }
}

func keychainQueryGet(securityDomain: RustStr, accessGroup: RustString?, hostname: RustStr) -> CzOutcome {
  let securityDomain = securityDomain.toString()
  let accessGroup = accessGroup?.toString()
  let hostname = hostname.toString()

  return CzOutcome {
    absent(try KeychainQuery().get(securityDomain: securityDomain, accessGroup: accessGroup, hostname: hostname))
  }
}

func keychainQueryList(securityDomain: RustStr, accessGroup: RustString?) -> CzOutcome {
  let securityDomain = securityDomain.toString()
  let accessGroup = accessGroup?.toString()

  return CzOutcome { try KeychainQuery().list(securityDomain: securityDomain, accessGroup: accessGroup) }
}

func keychainQueryExists(securityDomain: RustStr, accessGroup: RustString?, hostname: RustStr) -> CzOutcome {
  let securityDomain = securityDomain.toString()
  let accessGroup = accessGroup?.toString()
  let hostname = hostname.toString()

  return CzOutcome {
    try KeychainQuery().exists(securityDomain: securityDomain, accessGroup: accessGroup, hostname: hostname)
  }
}

func sysctlByName(name: RustStr) -> CzOutcome {
  let name = name.toString()

  return CzOutcome { try Sysctl.byName(name) }
}

func procNetSnmp(snmp: RustStr, netstat: RustString?, snmp6: RustString?) -> CzOutcome {
  let snmp = snmp.toString()
  let netstat = netstat?.toString()
  let snmp6 = snmp6?.toString()

  return CzOutcome { ProcNetSnmp(snmp: snmp, netstat: netstat, snmp6: snmp6) }
}

func readProcNetSnmp(pid: Int32?) -> CzOutcome {
  CzOutcome { try ProcNetSnmp.read(pid: pid) }
}

func fileInfoAt(path: RustStr) -> CzOutcome {
  let path = path.toString()

  return CzOutcome { CzFileInfo(try File.info(path)) }
}

/// A `FileInfo`.
final class CzFileInfo: Sendable {
  let info: FileInfo

  init(_ info: FileInfo) {
    self.info = info
  }

  func mode() -> UInt16 { info.mode }
  func uid() -> Int64 { Int64(info.uid) }
  func gid() -> Int64 { Int64(info.gid) }
  func dev() -> Int64 { Int64(info.dev) }
  func ino() -> Int64 { Int64(info.ino) }
  func size() -> Int64 { Int64(info.size) }
  func path() -> String { info.path }
  func isDirectory() -> Bool { info.isDirectory }
  func isPipe() -> Bool { info.isPipe }
  func isSocket() -> Bool { info.isSocket }
  func isLink() -> Bool { info.isLink }
  func isRegularFile() -> Bool { info.isRegularFile }
  func isBlock() -> Bool { info.isBlock }
  func isChar() -> Bool { info.isChar }
}

extension CzOutcome {
  func terminal() -> CzTerminal { taken() }
  func fileInfo() -> CzFileInfo { taken() }

  func parentTerminal() -> CzTerminal {
    CzTerminal((taken() as (parent: Terminal, child: Terminal)).parent)
  }

  func childTerminal() -> CzTerminal {
    CzTerminal((taken() as (parent: Terminal, child: Terminal)).child)
  }

  func terminalSizeWidth() -> UInt16 {
    (taken() as Terminal.Size).width
  }

  func terminalSizeHeight() -> UInt16 {
    (taken() as Terminal.Size).height
  }

  // A `KeychainQueryResult`, field by field: Rust builds its own.

  func keychainQueryResultUsername() -> String {
    (taken() as KeychainQueryResult).username
  }

  func keychainQueryResultPassword() -> String {
    (taken() as KeychainQueryResult).password
  }

  func keychainQueryResultModifiedDate() -> Double {
    (taken() as KeychainQueryResult).modifiedDate.timeIntervalSince1970
  }

  func keychainQueryResultCreatedDate() -> Double {
    (taken() as KeychainQueryResult).createdDate.timeIntervalSince1970
  }

  /// The held `ProcNetSnmp`'s `tcp`, `udp`, `icmp`, `ip`, `udp6`, `icmp6` and
  /// `ip6` counters. The last three may be `Absent`.
  func procNetSnmpCounters() -> CzOutcome {
    let snmp: ProcNetSnmp = taken()

    return CzOutcome.holding([
      snmp.tcp, snmp.udp, snmp.icmp, snmp.ip, absent(snmp.udp6), absent(snmp.icmp6), absent(snmp.ip6),
    ])
  }

  /// The held `ProcNetSnmp.Counters`' `UInt64` fields, in Swift's order.
  func procNetSnmpCountersNumbers() -> RustVec<UInt64> {
    let counters: ProcNetSnmp.Counters = taken()

    return rustVec([counters.receivedPackets, counters.transmittedPackets])
  }

  /// The held `ProcNetSnmp.Counters`' `UInt64?` fields, in Swift's order,
  /// each a `UInt64` or `Absent`.
  func procNetSnmpCountersOptionalNumbers() -> CzOutcome {
    let counters: ProcNetSnmp.Counters = taken()
    let numbers = [
      counters.receivedBytes, counters.transmittedBytes, counters.receivedDrops, counters.transmittedDrops,
      counters.receivedErrors, counters.transmittedErrors,
    ]

    return CzOutcome.holding(numbers.map { absent($0) })
  }

  /// A `Stat`'s unsigned fields, in Swift's order.
  func statUnsigned() -> RustVec<UInt64> {
    let stat: ContainerizationOS.Stat = taken()
    return rustVec([
      stat.dev, stat.ino, UInt64(stat.mode), stat.nlink, UInt64(stat.uid), UInt64(stat.gid), stat.rdev,
    ])
  }

  /// A `Stat`'s signed fields, in Swift's order, with each `TimeSpec` as its
  /// seconds and nanoseconds.
  func statSigned() -> RustVec<Int64> {
    let stat: ContainerizationOS.Stat = taken()
    return rustVec([
      stat.size, stat.blksize, stat.blocks,
      stat.atime.seconds, Int64(stat.atime.nanoseconds),
      stat.mtime.seconds, Int64(stat.mtime.nanoseconds),
      stat.ctime.seconds, Int64(stat.ctime.nanoseconds),
    ])
  }
}

/// For a unit test: `Binfmt.path`.
func binfmtPath() -> String {
  Binfmt.path
}

/// For a unit test: `Binfmt.Entry.amd64()`'s fields, in Swift's order.
func binfmtEntryAmd64() -> RustVec<RustString> {
  let entry = Binfmt.Entry.amd64()

  return rustStrings([entry.name, entry.type, entry.offset, entry.magic, entry.mask, entry.flags])
}
