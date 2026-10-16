//===----------------------------------------------------------------------===//
// The OCI runtime spec's cgroup resources, both ways: memory, CPU, block I/O,
// device rules and the rest. The Rust side is
// `bridge::accessors::spec::resources`.
//===----------------------------------------------------------------------===//

import ContainerizationOCI
import Foundation

// MARK: Rust to Swift

extension LinuxResources {
  init(_ resources: RustLinuxResourcesRef) {
    self.init(
      devices: list(resources.devicesLen()) { LinuxDeviceCgroup(resources.devicesAt($0)) },
      memory: resources.hasMemory() ? LinuxMemory(resources.memory()) : nil,
      cpu: resources.hasCpu() ? LinuxCPU(resources.cpu()) : nil,
      pids: resources.pidsLimit().map { LinuxPids(limit: $0) },
      blockIO: resources.hasBlockIO() ? LinuxBlockIO(resources.blockIO()) : nil,
      hugepageLimits: list(resources.hugepageLimitsLen()) {
        LinuxHugepageLimit(
          pagesize: resources.hugepageLimitPagesizeAt($0).toString(),
          limit: resources.hugepageLimitLimitAt($0)
        )
      },
      network: resources.hasNetwork()
        ? LinuxNetwork(
          classID: resources.networkClassID(),
          priorities: list(resources.networkPrioritiesLen()) {
            LinuxInterfacePriority(
              name: resources.networkPriorityNameAt($0).toString(),
              priority: resources.networkPriorityAt($0)
            )
          }
        ) : nil,
      rdma: resources.hasRdma()
        ? Dictionary(
          list(resources.rdmaLen()) {
            (
              resources.rdmaKeyAt($0).toString(),
              LinuxRdma(hcsHandles: resources.rdmaHcsHandlesAt($0), hcaObjects: resources.rdmaHcaObjectsAt($0))
            )
          },
          uniquingKeysWith: { _, last in last }
        ) : nil
    )
    // The initializer can't make `unified` nil.
    unified =
      resources.hasUnified()
      ? dictionary(resources.unifiedLen(), resources.unifiedKeyAt, resources.unifiedValueAt) : nil
  }
}

extension LinuxMemory {
  init(_ memory: RustLinuxMemoryRef) {
    self.init(
      limit: memory.limit(),
      reservation: memory.reservation(),
      swap: memory.swap(),
      kernel: memory.kernel(),
      kernelTCP: memory.kernelTCP(),
      swappiness: memory.swappiness(),
      disableOOMKiller: memory.disableOOMKiller(),
      useHierarchy: memory.useHierarchy(),
      checkBeforeUpdate: memory.checkBeforeUpdate()
    )
  }
}

extension LinuxCPU {
  init(_ cpu: RustLinuxCPURef) {
    self.init(
      shares: cpu.shares(),
      quota: cpu.quota(),
      burst: cpu.burst(),
      period: cpu.period(),
      realtimeRuntime: cpu.realtimeRuntime(),
      realtimePeriod: cpu.realtimePeriod(),
      cpus: cpu.cpus().toString(),
      mems: cpu.mems().toString(),
      idle: cpu.idle()
    )
  }
}

extension LinuxBlockIO {
  init(_ blockIO: RustLinuxBlockIORef) {
    func throttle(_ kind: ThrottleKind) -> [LinuxThrottleDevice] {
      list(blockIO.throttleLen(kind)) {
        LinuxThrottleDevice(
          major: blockIO.throttleMajorAt(kind, $0),
          minor: blockIO.throttleMinorAt(kind, $0),
          rate: blockIO.throttleRateAt(kind, $0)
        )
      }
    }

    self.init(
      weight: blockIO.weight(),
      leafWeight: blockIO.leafWeight(),
      weightDevice: list(blockIO.weightDeviceLen()) {
        LinuxWeightDevice(
          major: blockIO.weightDeviceMajorAt($0),
          minor: blockIO.weightDeviceMinorAt($0),
          weight: blockIO.weightDeviceWeightAt($0),
          leafWeight: blockIO.weightDeviceLeafWeightAt($0)
        )
      },
      throttleReadBpsDevice: throttle(.ReadBps),
      throttleWriteBpsDevice: throttle(.WriteBps),
      throttleReadIOPSDevice: throttle(.ReadIops),
      throttleWriteIOPSDevice: throttle(.WriteIops)
    )
  }
}

extension LinuxDeviceCgroup {
  init(_ device: RustLinuxDeviceCgroupRef) {
    self.init(
      allow: device.allow(),
      type: device.deviceType().toString(),
      major: device.major(),
      minor: device.minor(),
      access: device.access()?.toString()
    )
  }
}

// MARK: Swift to Rust

extension CzOutcome {
  // `LinuxResources`.

  func resourcesDevices() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).devices)
  }

  func resourcesMemory() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).memory)
  }

  func resourcesCpu() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).cpu)
  }

  func resourcesPids() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).pids)
  }

  func resourcesBlockIO() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).blockIO)
  }

  func resourcesHugepageLimits() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).hugepageLimits)
  }

  func resourcesNetwork() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).network)
  }

  func resourcesRdma() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).rdma)
  }

  func resourcesUnified() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxResources).unified)
  }

  // The types only `LinuxResources` holds.

  func memoryLimit() -> Int64? {
    (taken() as LinuxMemory).limit
  }

  func memoryReservation() -> Int64? {
    (taken() as LinuxMemory).reservation
  }

  func memorySwap() -> Int64? {
    (taken() as LinuxMemory).swap
  }

  func memoryKernel() -> Int64? {
    (taken() as LinuxMemory).kernel
  }

  func memoryKernelTCP() -> Int64? {
    (taken() as LinuxMemory).kernelTCP
  }

  func memorySwappiness() -> UInt64? {
    (taken() as LinuxMemory).swappiness
  }

  func memoryDisableOOMKiller() -> Bool? {
    (taken() as LinuxMemory).disableOOMKiller
  }

  func memoryUseHierarchy() -> Bool? {
    (taken() as LinuxMemory).useHierarchy
  }

  func memoryCheckBeforeUpdate() -> Bool? {
    (taken() as LinuxMemory).checkBeforeUpdate
  }

  func cpuShares() -> UInt64? {
    (taken() as LinuxCPU).shares
  }

  func cpuQuota() -> Int64? {
    (taken() as LinuxCPU).quota
  }

  func cpuBurst() -> UInt64? {
    (taken() as LinuxCPU).burst
  }

  func cpuPeriod() -> UInt64? {
    (taken() as LinuxCPU).period
  }

  func cpuRealtimeRuntime() -> Int64? {
    (taken() as LinuxCPU).realtimeRuntime
  }

  func cpuRealtimePeriod() -> Int64? {
    (taken() as LinuxCPU).realtimePeriod
  }

  func cpuCpus() -> String {
    (taken() as LinuxCPU).cpus
  }

  func cpuMems() -> String {
    (taken() as LinuxCPU).mems
  }

  func cpuIdle() -> Int64? {
    (taken() as LinuxCPU).idle
  }

  func pidsLimit() -> Int64 {
    (taken() as LinuxPids).limit
  }

  func blockIOWeight() -> UInt16? {
    (taken() as LinuxBlockIO).weight
  }

  func blockIOLeafWeight() -> UInt16? {
    (taken() as LinuxBlockIO).leafWeight
  }

  func blockIOWeightDevice() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxBlockIO).weightDevice)
  }

  func blockIOThrottle(kind: ThrottleKind) -> CzOutcome {
    let blockIO: LinuxBlockIO = taken()
    let devices: [LinuxThrottleDevice] =
      switch kind {
      case .ReadBps: blockIO.throttleReadBpsDevice
      case .WriteBps: blockIO.throttleWriteBpsDevice
      case .ReadIops: blockIO.throttleReadIOPSDevice
      case .WriteIops: blockIO.throttleWriteIOPSDevice
      }

    return CzOutcome.holding(devices)
  }

  func weightDeviceMajor() -> Int64 {
    (taken() as LinuxWeightDevice).major
  }

  func weightDeviceMinor() -> Int64 {
    (taken() as LinuxWeightDevice).minor
  }

  func weightDeviceWeight() -> UInt16? {
    (taken() as LinuxWeightDevice).weight
  }

  func weightDeviceLeafWeight() -> UInt16? {
    (taken() as LinuxWeightDevice).leafWeight
  }

  func throttleDeviceMajor() -> Int64 {
    (taken() as LinuxThrottleDevice).major
  }

  func throttleDeviceMinor() -> Int64 {
    (taken() as LinuxThrottleDevice).minor
  }

  func throttleDeviceRate() -> UInt64 {
    (taken() as LinuxThrottleDevice).rate
  }

  func hugepageLimitPagesize() -> String {
    (taken() as LinuxHugepageLimit).pagesize
  }

  func hugepageLimitLimit() -> UInt64 {
    (taken() as LinuxHugepageLimit).limit
  }

  func networkClassID() -> UInt32? {
    (taken() as LinuxNetwork).classID
  }

  func networkPriorities() -> CzOutcome {
    CzOutcome.holding((taken() as LinuxNetwork).priorities)
  }

  func interfacePriorityName() -> String {
    (taken() as LinuxInterfacePriority).name
  }

  func interfacePriorityPriority() -> UInt32 {
    (taken() as LinuxInterfacePriority).priority
  }

  func rdmaHcsHandles() -> UInt32? {
    (taken() as LinuxRdma).hcsHandles
  }

  func rdmaHcaObjects() -> UInt32? {
    (taken() as LinuxRdma).hcaObjects
  }

  // `LinuxDeviceCgroup`.

  func deviceCgroupAllow() -> Bool {
    (taken() as LinuxDeviceCgroup).allow
  }

  func deviceCgroupType() -> String {
    (taken() as LinuxDeviceCgroup).type
  }

  func deviceCgroupMajor() -> Int64? {
    (taken() as LinuxDeviceCgroup).major
  }

  func deviceCgroupMinor() -> Int64? {
    (taken() as LinuxDeviceCgroup).minor
  }

  func deviceCgroupAccess() -> String? {
    (taken() as LinuxDeviceCgroup).access
  }
}
