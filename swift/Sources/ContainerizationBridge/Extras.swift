//===----------------------------------------------------------------------===//
// ContainerizationExtras' `ProgressEvent`, `ProgressHandler` and `ProxyUtils`.
//===----------------------------------------------------------------------===//

import ContainerizationExtras
import Foundation

extension ProgressEvent {
  init(_ kind: ProgressKind, _ value: Int64) {
    self =
      switch kind {
      case .Items: .addItems(Int(value))
      case .TotalItems: .addTotalItems(Int(value))
      case .Size: .addSize(value)
      case .TotalSize: .addTotalSize(value)
      }
  }
}

/// Rust's `ProgressHandler?`. Swift may call it from several threads at once,
/// and Rust's is `Sync`. Swift's handler is `async`, so Rust's can run off the
/// cooperative pool, where it may call back into Swift.
func progressHandler(_ handler: RustProgressHandler) -> ProgressHandler? {
  guard handler.isSome() else {
    return nil
  }
  nonisolated(unsafe) let handler = handler

  return { events in
    await offPool { call(handler, with: events) }
  }
}

private func call(_ handler: RustProgressHandler, with events: [ProgressEvent]) {
  let kinds = RustVec<ProgressKind>()
  let values = RustVec<Int64>()
  for event in events {
    switch event {
    case .addItems(let value):
      kinds.push(value: .Items)
      values.push(value: Int64(value))
    case .addTotalItems(let value):
      kinds.push(value: .TotalItems)
      values.push(value: Int64(value))
    case .addSize(let value):
      kinds.push(value: .Size)
      values.push(value: value)
    case .addTotalSize(let value):
      kinds.push(value: .TotalSize)
      values.push(value: value)
    }
  }
  handler.call(kinds, values)
}

func progressEventEvent(kind: ProgressKind, value: Int64) -> CzOutcome {
  let event = ProgressEvent(kind, value)

  return CzOutcome { event.event }
}

/// The outcome holds the URL's `absoluteString`, or `Absent`.
func proxyFromEnvironment(
  scheme: RustString?,
  host: RustStr,
  hasEnv: Bool,
  envKeys: RustVec<RustString>,
  envValues: RustVec<RustString>
) -> CzOutcome {
  let scheme = scheme?.toString()
  let host = host.toString()
  let env = hasEnv ? dictionary(envKeys, envValues) : nil

  return CzOutcome {
    let proxy =
      if let env {
        ProxyUtils.proxyFromEnvironment(scheme: scheme, host: host, env: env)
      } else {
        ProxyUtils.proxyFromEnvironment(scheme: scheme, host: host)
      }

    return absent(proxy?.absoluteString)
  }
}
