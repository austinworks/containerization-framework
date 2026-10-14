//===----------------------------------------------------------------------===//
// Containerization is async throughout; the bridge is not.
//===----------------------------------------------------------------------===//

import Foundation

/// Runs an async body in a detached task and blocks until it finishes.
///
/// Blocking a thread of Swift's cooperative pool could leave no thread to run
/// the body. Rust calls in on its own threads, and Swift calls the Rust
/// closures it awaits through `offPool`, so neither blocks the pool. Swift
/// calls Rust's synchronous configuration closures on the pool, which is one
/// reason their docs say not to call back.
///
/// The task releases `body` before it signals, so nothing holds it after this
/// returns and a caller may pass a closure from `withoutActuallyEscaping`.
func blocking<T>(_ body: @escaping @Sendable () async throws -> T) throws -> T {
  let semaphore = DispatchSemaphore(value: 0)
  nonisolated(unsafe) var outcome: Result<T, any Error>?
  nonisolated(unsafe) var pending: (@Sendable () async throws -> T)? = body

  Task.detached {
    if let body = pending.take() {
      do {
        outcome = .success(try await body())
      } catch {
        outcome = .failure(error)
      }
    }
    semaphore.signal()
  }

  semaphore.wait()

  guard let outcome else {
    throw BridgeError.noOutcome
  }

  return try outcome.get()
}

/// Where Swift runs the Rust closures it awaits. It is concurrent, because
/// Swift may await several at once, and Dispatch adds a thread when one blocks.
private let rustClosures = DispatchQueue(label: "ContainerizationBridge.rustClosures", attributes: .concurrent)

/// Runs a Rust closure that Swift awaits off Swift's cooperative pool, and
/// waits for it without blocking.
///
/// The closure may call back into Swift through `blocking`. On the pool, each
/// such call would park a pool thread while its task needs another, and with
/// every thread parked nothing could run.
func offPool<T: Sendable>(_ body: @escaping @Sendable () -> T) async -> T {
  await withCheckedContinuation { continuation in
    rustClosures.async {
      continuation.resume(returning: body())
    }
  }
}
