//===----------------------------------------------------------------------===//
// The OCI runtime spec's lifecycle hooks, both ways. The Rust side is
// `bridge::accessors::spec::hooks`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension Hook {
  init(_ hook: RustHookRef) {
    self.init(
      path: hook.path().toString(),
      args: strings(hook.argsLen(), hook.argsAt),
      env: strings(hook.envLen(), hook.envAt),
      timeout: hook.timeout()
    )
  }
}

extension Hooks {
  init(_ hooks: RustHooksRef) {
    func of(_ kind: HookKind) -> [Hook] {
      list(hooks.hooksLen(kind)) { Hook(hooks.hooksAt(kind, $0)) }
    }

    self.init(
      prestart: of(.Prestart),
      createRuntime: of(.CreateRuntime),
      createContainer: of(.CreateContainer),
      startContainer: of(.StartContainer),
      poststart: of(.Poststart),
      poststop: of(.Poststop)
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  func hookPath() -> String {
    (taken() as Hook).path
  }

  func hookArgs() -> RustVec<RustString> {
    rustStrings((taken() as Hook).args)
  }

  func hookEnv() -> RustVec<RustString> {
    rustStrings((taken() as Hook).env)
  }

  func hookTimeout() -> Int? {
    (taken() as Hook).timeout
  }

  func hooksOf(kind: HookKind) -> CzOutcome {
    let hooks: Hooks = taken()
    let list: [Hook] =
      switch kind {
      case .Prestart: hooks.prestart
      case .CreateRuntime: hooks.createRuntime
      case .CreateContainer: hooks.createContainer
      case .StartContainer: hooks.startContainer
      case .Poststart: hooks.poststart
      case .Poststop: hooks.poststop
      }

    return CzOutcome.holding(list)
  }
}
