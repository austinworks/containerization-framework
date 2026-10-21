//===----------------------------------------------------------------------===//
// CloudHypervisor's `DiskConfig`, read back by Rust field by field. Its
// `ImageType` crosses as its `rawValue`.
//===----------------------------------------------------------------------===//

import CloudHypervisor

extension CzOutcome {
  func diskConfigPath() -> String {
    (taken() as CloudHypervisor.DiskConfig).path
  }

  func diskConfigReadonly() -> Bool? {
    (taken() as CloudHypervisor.DiskConfig).readonly
  }

  func diskConfigDirect() -> Bool? {
    (taken() as CloudHypervisor.DiskConfig).direct
  }

  func diskConfigIommu() -> Bool? {
    (taken() as CloudHypervisor.DiskConfig).iommu
  }

  func diskConfigId() -> String? {
    (taken() as CloudHypervisor.DiskConfig).id
  }

  func diskConfigPciSegment() -> UInt16? {
    (taken() as CloudHypervisor.DiskConfig).pciSegment
  }

  func diskConfigImageType() -> String? {
    (taken() as CloudHypervisor.DiskConfig).imageType?.rawValue
  }
}

/// For a unit test: `ImageType`'s raw values, in Swift's order.
func imageTypeRawValues() -> RustVec<RustString> {
  let imageTypes: [CloudHypervisor.ImageType] = [.raw, .qcow2, .fixedVhd, .vhdx, .unknown]

  return rustStrings(imageTypes.map(\.rawValue))
}
