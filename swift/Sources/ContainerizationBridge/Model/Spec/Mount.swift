//===----------------------------------------------------------------------===//
// The OCI runtime spec's mounts, both ways. The Rust side is
// `bridge::accessors::spec::mount`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension ContainerizationOCI.Mount {
  init(_ mount: RustOciMountRef) {
    self.init(
      type: mount.mountType().toString(),
      source: mount.source().toString(),
      destination: mount.destination().toString(),
      options: strings(mount.optionsLen(), mount.optionsAt),
      uidMappings: mount.hasUidMappings()
        ? list(mount.uidMappingsLen()) { LinuxIDMapping(mount.uidMappingsAt($0)) } : nil,
      gidMappings: mount.hasGidMappings()
        ? list(mount.gidMappingsLen()) { LinuxIDMapping(mount.gidMappingsAt($0)) } : nil
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  func ociMountType() -> String {
    (taken() as ContainerizationOCI.Mount).type
  }

  func ociMountSource() -> String {
    (taken() as ContainerizationOCI.Mount).source
  }

  func ociMountDestination() -> String {
    (taken() as ContainerizationOCI.Mount).destination
  }

  func ociMountOptions() -> RustVec<RustString> {
    rustStrings((taken() as ContainerizationOCI.Mount).options)
  }

  func ociMountUidMappings() -> CzOutcome {
    CzOutcome.holding((taken() as ContainerizationOCI.Mount).uidMappings)
  }

  func ociMountGidMappings() -> CzOutcome {
    CzOutcome.holding((taken() as ContainerizationOCI.Mount).gidMappings)
  }
}
