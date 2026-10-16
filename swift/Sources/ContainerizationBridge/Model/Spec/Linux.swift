//===----------------------------------------------------------------------===//
// The OCI runtime spec's Linux section, both ways: its namespaces, ID
// mappings, devices and personality. The Rust side is
// `bridge::accessors::spec::linux`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension Linux {
  init(_ linux: RustLinuxRef) {
    self.init(
      uidMappings: list(linux.uidMappingsLen()) { LinuxIDMapping(linux.uidMappingsAt($0)) },
      gidMappings: list(linux.gidMappingsLen()) { LinuxIDMapping(linux.gidMappingsAt($0)) },
      sysctl: linux.hasSysctl() ? dictionary(linux.sysctlLen(), linux.sysctlKeyAt, linux.sysctlValueAt) : nil,
      resources: linux.hasResources() ? LinuxResources(linux.resources()) : nil,
      cgroupsPath: linux.cgroupsPath().toString(),
      namespaces: list(linux.namespacesLen()) {
        LinuxNamespace(type: swiftCase(linux.namespaceTypeAt($0)), path: linux.namespacePathAt($0).toString())
      },
      devices: list(linux.devicesLen()) { LinuxDevice(linux.devicesAt($0)) },
      seccomp: linux.hasSeccomp() ? LinuxSeccomp(linux.seccomp()) : nil,
      rootfsPropagation: linux.rootfsPropagation().toString(),
      maskedPaths: strings(linux.maskedPathsLen(), linux.maskedPathsAt),
      readonlyPaths: strings(linux.readonlyPathsLen(), linux.readonlyPathsAt),
      mountLabel: linux.mountLabel().toString(),
      personality: linux.hasPersonality()
        ? LinuxPersonality(
          domain: swiftCase(linux.personalityDomain()),
          flags: strings(linux.personalityFlagsLen(), linux.personalityFlagsAt)
        ) : nil
    )
  }
}

extension LinuxIDMapping {
  init(_ mapping: RustLinuxIDMappingRef) {
    self.init(containerID: mapping.containerID(), hostID: mapping.hostID(), size: mapping.size())
  }
}

extension LinuxDevice {
  init(_ device: RustLinuxDeviceRef) {
    self.init(
      path: device.path().toString(),
      type: device.deviceType().toString(),
      major: device.major(),
      minor: device.minor(),
      fileMode: device.fileMode(),
      uid: device.uid(),
      gid: device.gid()
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  // `Linux`.

  func linuxUidMappings() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).uidMappings)
  }

  func linuxGidMappings() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).gidMappings)
  }

  func linuxSysctl() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).sysctl)
  }

  func linuxResources() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).resources)
  }

  func linuxCgroupsPath() -> String {
    (taken() as Linux).cgroupsPath
  }

  func linuxNamespaces() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).namespaces)
  }

  func linuxDevices() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).devices)
  }

  func linuxSeccomp() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).seccomp)
  }

  func linuxRootfsPropagation() -> String {
    (taken() as Linux).rootfsPropagation
  }

  func linuxMaskedPaths() -> RustVec<RustString> {
    rustStrings((taken() as Linux).maskedPaths)
  }

  func linuxReadonlyPaths() -> RustVec<RustString> {
    rustStrings((taken() as Linux).readonlyPaths)
  }

  func linuxMountLabel() -> String {
    (taken() as Linux).mountLabel
  }

  func linuxPersonality() -> CzOutcome {
    CzOutcome.holding((taken() as Linux).personality)
  }

  // `LinuxNamespace` and `LinuxIDMapping`.

  func namespaceType() -> String {
    (taken() as LinuxNamespace).type.rawValue
  }

  func namespacePath() -> String {
    (taken() as LinuxNamespace).path
  }

  func idMappingContainerID() -> UInt32 {
    (taken() as LinuxIDMapping).containerID
  }

  func idMappingHostID() -> UInt32 {
    (taken() as LinuxIDMapping).hostID
  }

  func idMappingSize() -> UInt32 {
    (taken() as LinuxIDMapping).size
  }

  // `LinuxDevice`.

  func devicePath() -> String {
    (taken() as LinuxDevice).path
  }

  func deviceType() -> String {
    (taken() as LinuxDevice).type
  }

  func deviceMajor() -> Int64 {
    (taken() as LinuxDevice).major
  }

  func deviceMinor() -> Int64 {
    (taken() as LinuxDevice).minor
  }

  func deviceFileMode() -> UInt32? {
    (taken() as LinuxDevice).fileMode
  }

  func deviceUid() -> UInt32? {
    (taken() as LinuxDevice).uid
  }

  func deviceGid() -> UInt32? {
    (taken() as LinuxDevice).gid
  }

  // `LinuxPersonality`.

  func personalityDomain() -> String {
    (taken() as LinuxPersonality).domain.rawValue
  }

  func personalityFlags() -> RustVec<RustString> {
    rustStrings((taken() as LinuxPersonality).flags)
  }
}
