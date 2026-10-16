//===----------------------------------------------------------------------===//
// The OCI runtime spec's seccomp profile and its syscall rules, both ways.
// The Rust side is `bridge::accessors::spec::seccomp`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension LinuxSeccomp {
  init(_ seccomp: RustLinuxSeccompRef) {
    self.init(
      defaultAction: swiftCase(seccomp.defaultAction()),
      defaultErrnoRet: seccomp.defaultErrnoRet(),
      architectures: list(seccomp.architecturesLen()) { swiftCase(seccomp.architecturesAt($0)) },
      flags: list(seccomp.flagsLen()) { swiftCase(seccomp.flagsAt($0)) },
      listenerPath: seccomp.listenerPath().toString(),
      listenerMetadata: seccomp.listenerMetadata().toString(),
      syscalls: list(seccomp.syscallsLen()) { LinuxSyscall(seccomp.syscallsAt($0)) }
    )
  }
}

extension LinuxSyscall {
  init(_ syscall: RustLinuxSyscallRef) {
    self.init(
      names: strings(syscall.namesLen(), syscall.namesAt),
      action: swiftCase(syscall.action()),
      errnoRet: syscall.errnoRet(),
      args: list(syscall.argsLen()) {
        LinuxSeccompArg(
          index: syscall.argIndexAt($0),
          value: syscall.argValueAt($0),
          valueTwo: syscall.argValueTwoAt($0),
          op: swiftCase(syscall.argOpAt($0))
        )
      }
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  func seccompDefaultAction() -> String {
    (taken() as LinuxSeccomp).defaultAction.rawValue
  }

  func seccompDefaultErrnoRet() -> UInt? {
    (taken() as LinuxSeccomp).defaultErrnoRet
  }

  func seccompArchitectures() -> RustVec<RustString> {
    rustStrings((taken() as LinuxSeccomp).architectures.map(\.rawValue))
  }

  func seccompFlags() -> RustVec<RustString> {
    rustStrings((taken() as LinuxSeccomp).flags.map(\.rawValue))
  }

  func seccompListenerPath() -> String {
    (taken() as LinuxSeccomp).listenerPath
  }

  func seccompListenerMetadata() -> String {
    (taken() as LinuxSeccomp).listenerMetadata
  }

  func seccompSyscalls() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxSeccomp).syscalls)
  }

  func syscallNames() -> RustVec<RustString> {
    rustStrings((taken() as LinuxSyscall).names)
  }

  func syscallAction() -> String {
    (taken() as LinuxSyscall).action.rawValue
  }

  func syscallErrnoRet() -> UInt? {
    (taken() as LinuxSyscall).errnoRet
  }

  func syscallArgs() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxSyscall).args)
  }

  func seccompArgIndex() -> UInt {
    (taken() as LinuxSeccompArg).index
  }

  func seccompArgValue() -> UInt64 {
    (taken() as LinuxSeccompArg).value
  }

  func seccompArgValueTwo() -> UInt64 {
    (taken() as LinuxSeccompArg).valueTwo
  }

  func seccompArgOp() -> String {
    (taken() as LinuxSeccompArg).op.rawValue
  }
}
