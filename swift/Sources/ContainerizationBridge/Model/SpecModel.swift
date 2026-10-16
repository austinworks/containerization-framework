//===----------------------------------------------------------------------===//
// The OCI runtime spec's types: Rust's read into ContainerizationOCI's, and
// ContainerizationOCI's read back by Rust field by field. An enum crosses as
// its `rawValue`.
//
// This file holds `Spec` itself, `Root` and `RuntimeSpecVersion`. The types
// `Spec` holds are in `Spec/`, one file per file of
// `bridge::accessors::spec`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension Spec {
  init(_ spec: RustSpecRef) {
    self.init(
      version: spec.version().toString(),
      hooks: spec.hasHooks() ? Hooks(spec.hooks()) : nil,
      process: spec.hasProcess() ? ContainerizationOCI.Process(spec.process()) : nil,
      hostname: spec.hostname().toString(),
      domainname: spec.domainname().toString(),
      mounts: list(spec.mountsLen()) { ContainerizationOCI.Mount(spec.mountsAt($0)) },
      annotations: spec.hasAnnotations()
        ? dictionary(spec.annotationsLen(), spec.annotationKeyAt, spec.annotationValueAt) : nil,
      root: spec.hasRoot() ? Root(path: spec.rootPath().toString(), readonly: spec.rootReadonly()) : nil,
      linux: spec.hasLinux() ? Linux(spec.linux()) : nil
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  // `Spec`.

  func specVersion() -> String {
    (taken() as Spec).version
  }

  func specHooks() -> CzOutcome {
    CzOutcome.holding((taken() as Spec).hooks)
  }

  func specProcess() -> CzOutcome {
    CzOutcome.holding((taken() as Spec).process)
  }

  func specHostname() -> String {
    (taken() as Spec).hostname
  }

  func specDomainname() -> String {
    (taken() as Spec).domainname
  }

  func specMounts() -> CzOutcome {
    CzOutcome.holding((taken() as Spec).mounts)
  }

  func specAnnotations() -> CzOutcome {
    CzOutcome.holding((taken() as Spec).annotations)
  }

  func specRoot() -> CzOutcome {
    CzOutcome.holding((taken() as Spec).root)
  }

  func specLinux() -> CzOutcome {
    CzOutcome.holding((taken() as Spec).linux)
  }

  // `Root`.

  func rootPath() -> String {
    (taken() as Root).path
  }

  func rootReadonly() -> Bool {
    (taken() as Root).readonly
  }

  // `RuntimeSpecVersion`.

  func runtimeSpecVersionMajor() -> Int {
    (taken() as RuntimeSpecVersion).major
  }

  func runtimeSpecVersionMinor() -> Int {
    (taken() as RuntimeSpecVersion).minor
  }

  func runtimeSpecVersionPatch() -> Int {
    (taken() as RuntimeSpecVersion).patch
  }

  func runtimeSpecVersionDev() -> String {
    (taken() as RuntimeSpecVersion).dev
  }
}
