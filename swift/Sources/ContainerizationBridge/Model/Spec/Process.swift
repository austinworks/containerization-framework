//===----------------------------------------------------------------------===//
// The OCI runtime spec's process, its user, capabilities and rlimits, both
// ways. The Rust side is `bridge::accessors::spec::process`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension ContainerizationOCI.Process {
  init(_ process: RustProcessRef) {
    self.init(
      args: strings(process.argsLen(), process.argsAt),
      cwd: process.cwd().toString(),
      env: strings(process.envLen(), process.envAt),
      consoleSize: process.hasConsoleSize()
        ? Box(height: process.consoleHeight(), width: process.consoleWidth()) : nil,
      selinuxLabel: process.selinuxLabel().toString(),
      noNewPrivileges: process.noNewPrivileges(),
      commandLine: process.commandLine().toString(),
      oomScoreAdj: process.oomScoreAdj(),
      capabilities: process.hasCapabilities() ? ContainerizationOCI.LinuxCapabilities(process.capabilities()) : nil,
      apparmorProfile: process.apparmorProfile().toString(),
      user: ContainerizationOCI.User(process.user()),
      rlimits: list(process.rlimitsLen()) {
        POSIXRlimit(
          type: process.rlimitTypeAt($0).toString(),
          hard: process.rlimitHardAt($0),
          soft: process.rlimitSoftAt($0)
        )
      },
      terminal: process.terminal()
    )
  }
}

extension ContainerizationOCI.LinuxCapabilities {
  init(_ capabilities: RustOciLinuxCapabilitiesRef) {
    func set(_ set: CapabilitySet) -> [String]? {
      capabilities.hasSet(set) ? strings(capabilities.setLen(set)) { capabilities.setAt(set, $0) } : nil
    }

    self.init(
      bounding: set(.Bounding),
      effective: set(.Effective),
      inheritable: set(.Inheritable),
      permitted: set(.Permitted),
      ambient: set(.Ambient)
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  // `Process`.

  func processCwd() -> String {
    (taken() as ContainerizationOCI.Process).cwd
  }

  func processEnv() -> RustVec<RustString> {
    rustStrings((taken() as ContainerizationOCI.Process).env)
  }

  func processConsoleSize() -> CzOutcome {
    CzOutcome.holding((taken() as ContainerizationOCI.Process).consoleSize)
  }

  func processSelinuxLabel() -> String {
    (taken() as ContainerizationOCI.Process).selinuxLabel
  }

  func processNoNewPrivileges() -> Bool {
    (taken() as ContainerizationOCI.Process).noNewPrivileges
  }

  func processCommandLine() -> String {
    (taken() as ContainerizationOCI.Process).commandLine
  }

  func processOomScoreAdj() -> Int? {
    (taken() as ContainerizationOCI.Process).oomScoreAdj
  }

  func processCapabilities() -> CzOutcome {
    CzOutcome.holding((taken() as ContainerizationOCI.Process).capabilities)
  }

  func processApparmorProfile() -> String {
    (taken() as ContainerizationOCI.Process).apparmorProfile
  }

  func processUser() -> CzOutcome {
    CzOutcome.holding((taken() as ContainerizationOCI.Process).user)
  }

  func processRlimits() -> CzOutcome {
    CzOutcome.holding((taken() as ContainerizationOCI.Process).rlimits)
  }

  func processArgs() -> RustVec<RustString> {
    rustStrings((taken() as ContainerizationOCI.Process).args)
  }

  func processTerminal() -> Bool {
    (taken() as ContainerizationOCI.Process).terminal
  }

  // `Box`, whose fields are internal.

  func boxHeight() -> UInt {
    cast(Mirror(reflecting: taken() as Box).descendant("height"))
  }

  func boxWidth() -> UInt {
    cast(Mirror(reflecting: taken() as Box).descendant("width"))
  }

  // `User`.

  func userUid() -> UInt32 {
    (taken() as ContainerizationOCI.User).uid
  }

  func userGid() -> UInt32 {
    (taken() as ContainerizationOCI.User).gid
  }

  func userUmask() -> UInt32? {
    (taken() as ContainerizationOCI.User).umask
  }

  func userAdditionalGids() -> RustVec<UInt32> {
    rustVec((taken() as ContainerizationOCI.User).additionalGids)
  }

  func userUsername() -> String {
    (taken() as ContainerizationOCI.User).username
  }

  // `LinuxCapabilities`.

  func capabilitiesSet(set: CapabilitySet) -> CzOutcome {
    let capabilities: ContainerizationOCI.LinuxCapabilities = taken()
    let names: [String]? =
      switch set {
      case .Bounding: capabilities.bounding
      case .Effective: capabilities.effective
      case .Inheritable: capabilities.inheritable
      case .Permitted: capabilities.permitted
      case .Ambient: capabilities.ambient
      }

    return CzOutcome.holding(names)
  }

  // `POSIXRlimit`.

  func rlimitType() -> String {
    (taken() as POSIXRlimit).type
  }

  func rlimitHard() -> UInt64 {
    (taken() as POSIXRlimit).hard
  }

  func rlimitSoft() -> UInt64 {
    (taken() as POSIXRlimit).soft
  }
}
