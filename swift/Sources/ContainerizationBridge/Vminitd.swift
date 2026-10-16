//===----------------------------------------------------------------------===//
// `Vminitd`, from Containerization: the client of the agent running in the
// guest, which `VZVirtualMachineInstance.dialAgent()` returns.
//===----------------------------------------------------------------------===//

import Containerization
import ContainerizationExtras
import ContainerizationOCI
import ContainerizationOS
import Foundation

final class CzVminitd: Sendable {
  let agent: Vminitd

  init(_ agent: Vminitd) {
    self.agent = agent
  }

  func standardSetup() -> CzOutcome {
    let agent = agent

    return CzOutcome { try blocking { try await agent.standardSetup() } }
  }

  func close() -> CzOutcome {
    let agent = agent

    return CzOutcome { try blocking { try await agent.close() } }
  }

  func filesystemOperation(operation: FilesystemOperationKind, path: RustStr, containerID: RustString?) -> CzOutcome {
    let agent = agent
    let path = path.toString()
    let containerID = containerID?.toString()
    let operation: FilesystemOperation =
      switch operation {
      case .Freeze: .freeze
      case .Thaw: .thaw
      case .Trim: .trim
      }

    return CzOutcome {
      try blocking { try await agent.filesystemOperation(operation: operation, path: path, containerID: containerID) }
    }
  }

  func getenv(key: RustStr) -> CzOutcome {
    let agent = agent
    let key = key.toString()

    return CzOutcome { try blocking { try await agent.getenv(key: key) } }
  }

  func setenv(key: RustStr, value: RustStr) -> CzOutcome {
    let agent = agent
    let key = key.toString()
    let value = value.toString()

    return CzOutcome { try blocking { try await agent.setenv(key: key, value: value) } }
  }

  func mount(mount: RustOciMount) -> CzOutcome {
    let agent = agent
    let mount = ContainerizationOCI.Mount(mount)

    return CzOutcome { try blocking { try await agent.mount(mount) } }
  }

  func umount(path: RustStr, flags: Int32) -> CzOutcome {
    let agent = agent
    let path = path.toString()

    return CzOutcome { try blocking { try await agent.umount(path: path, flags: flags) } }
  }

  func mkdir(path: RustStr, all: Bool, perms: UInt32) -> CzOutcome {
    let agent = agent
    let path = path.toString()

    return CzOutcome { try blocking { try await agent.mkdir(path: path, all: all, perms: perms) } }
  }

  func kill(pid: Int32, signal: Int32) -> CzOutcome {
    let agent = agent

    return CzOutcome { try blocking { try await agent.kill(pid: pid, signal: signal) } }
  }

  func sync() -> CzOutcome {
    let agent = agent

    return CzOutcome { try blocking { try await agent.sync() } }
  }

  /// `options` stands for `nil` when `hasOptions` is false.
  func createProcess(
    id: RustStr,
    containerID: RustString?,
    stdinPort: UInt32?,
    stdoutPort: UInt32?,
    stderrPort: UInt32?,
    ociRuntimePath: RustString?,
    configuration: RustSpec,
    hasOptions: Bool,
    options: RustVec<UInt8>
  ) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()
    let ociRuntimePath = ociRuntimePath?.toString()
    let configuration = ContainerizationOCI.Spec(configuration)
    let options = hasOptions ? Data(options) : nil

    return CzOutcome {
      try blocking {
        try await agent.createProcess(
          id: id,
          containerID: containerID,
          stdinPort: stdinPort,
          stdoutPort: stdoutPort,
          stderrPort: stderrPort,
          ociRuntimePath: ociRuntimePath,
          configuration: configuration,
          options: options
        )
      }
    }
  }

  func startProcess(id: RustStr, containerID: RustString?) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()

    return CzOutcome { try blocking { try await agent.startProcess(id: id, containerID: containerID) } }
  }

  func signalProcess(id: RustStr, containerID: RustString?, signal: Int32) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()

    return CzOutcome {
      try blocking { try await agent.signalProcess(id: id, containerID: containerID, signal: signal) }
    }
  }

  func resizeProcess(id: RustStr, containerID: RustString?, columns: UInt32, rows: UInt32) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()

    return CzOutcome {
      try blocking { try await agent.resizeProcess(id: id, containerID: containerID, columns: columns, rows: rows) }
    }
  }

  func waitProcess(id: RustStr, containerID: RustString?, timeoutInSeconds: Int64?) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()

    return CzOutcome {
      try blocking { try await agent.waitProcess(id: id, containerID: containerID, timeoutInSeconds: timeoutInSeconds) }
    }
  }

  func deleteProcess(id: RustStr, containerID: RustString?) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()

    return CzOutcome { try blocking { try await agent.deleteProcess(id: id, containerID: containerID) } }
  }

  func closeProcessStdin(id: RustStr, containerID: RustString?) -> CzOutcome {
    let agent = agent
    let id = id.toString()
    let containerID = containerID?.toString()

    return CzOutcome { try blocking { try await agent.closeProcessStdin(id: id, containerID: containerID) } }
  }

  func up(name: RustStr, mtu: UInt32?) -> CzOutcome {
    let agent = agent
    let name = name.toString()

    return CzOutcome { try blocking { try await agent.up(name: name, mtu: mtu) } }
  }

  func down(name: RustStr) -> CzOutcome {
    let agent = agent
    let name = name.toString()

    return CzOutcome { try blocking { try await agent.down(name: name) } }
  }

  func addressAdd(name: RustStr, address: RustInterfaceAddress) -> CzOutcome {
    let agent = agent
    let name = name.toString()

    return CzOutcome {
      let address = try InterfaceAddress(address)

      return try blocking { try await agent.addressAdd(name: name, address: address) }
    }
  }

  func routeAddLink(name: RustStr, route: RustLinkRoute) -> CzOutcome {
    let agent = agent
    let name = name.toString()
    let route = LinkRoute(route)

    return CzOutcome { try blocking { try await agent.routeAddLink(name: name, route: route) } }
  }

  func routeAddDefault(name: RustStr, route: RustDefaultRoute) -> CzOutcome {
    let agent = agent
    let name = name.toString()
    let route = DefaultRoute(route)

    return CzOutcome { try blocking { try await agent.routeAddDefault(name: name, route: route) } }
  }

  func configureDNS(config: RustDns, location: RustStr) -> CzOutcome {
    let agent = agent
    let config = DNS(config)
    let location = location.toString()

    return CzOutcome { try blocking { try await agent.configureDNS(config: config, location: location) } }
  }

  func configureHosts(config: RustHosts, location: RustStr) -> CzOutcome {
    let agent = agent
    let config = Hosts(config)
    let location = location.toString()

    return CzOutcome { try blocking { try await agent.configureHosts(config: config, location: location) } }
  }

  func containerStatistics(containerIDs: RustVec<RustString>, categories: Int64) -> CzOutcome {
    let agent = agent
    let containerIDs = strings(containerIDs)
    let categories = StatCategory(rawValue: Int(categories))

    return CzOutcome {
      try blocking { try await agent.containerStatistics(containerIDs: containerIDs, categories: categories) }
    }
  }

  func setupEmulator(binaryPath: RustStr, configuration: RustBinfmtEntry) -> CzOutcome {
    let agent = agent
    let binaryPath = binaryPath.toString()
    // `Binfmt.Entry` isn't `Sendable`, though it holds only `String`s.
    nonisolated(unsafe) let configuration = Binfmt.Entry(configuration)

    return CzOutcome {
      try blocking { try await agent.setupEmulator(binaryPath: binaryPath, configuration: configuration) }
    }
  }

  func setTime(sec: Int64, usec: Int32) -> CzOutcome {
    let agent = agent

    return CzOutcome { try blocking { try await agent.setTime(sec: sec, usec: usec) } }
  }

  func sysctl(keys: RustVec<RustString>, values: RustVec<RustString>) -> CzOutcome {
    let agent = agent
    let settings = dictionary(keys, values)

    return CzOutcome { try blocking { try await agent.sysctl(settings: settings) } }
  }

  func stat(root: RustStr, path: RustStr) -> CzOutcome {
    let agent = agent
    let root = root.toString()
    let path = path.toString()

    return CzOutcome { try blocking { try await agent.stat(root: root, path: path) } }
  }

  func enableRosetta() -> CzOutcome {
    let agent = agent

    return CzOutcome { try blocking { try await agent.enableRosetta() } }
  }

  func relaySocket(port: UInt32, configuration: CzUnixSocketConfiguration) -> CzOutcome {
    let agent = agent
    let configuration = configuration.configuration

    return CzOutcome { try blocking { try await agent.relaySocket(port: port, configuration: configuration) } }
  }

  func stopSocketRelay(configuration: CzUnixSocketConfiguration) -> CzOutcome {
    let agent = agent
    let configuration = configuration.configuration

    return CzOutcome { try blocking { try await agent.stopSocketRelay(configuration: configuration) } }
  }
}

extension CzVirtualMachineInstance {
  func dialAgent() -> CzOutcome {
    let instance = instance

    return CzOutcome { CzVminitd(try blocking { try await instance.dialAgent() }) }
  }
}

extension CzOutcome {
  func vminitd() -> CzVminitd { taken() }
}
