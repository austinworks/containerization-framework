//===----------------------------------------------------------------------===//
// ContainerizationOS: `FileDescriptorOps`.
//
// Rust passes each descriptor as its number and keeps it, so Swift wraps it
// without closing it. A descriptor Swift opens crosses back as an `Int32`,
// which Rust owns. A `FilePermissions?` crosses as a flag and its raw value.
//===----------------------------------------------------------------------===//

import ContainerizationOS
import SystemPackage

func fdOpsEntryType(fd: Int32, name: RustStr) -> CzOutcome {
  CzOutcome { absent(try FileDescriptorOps.entryType(FileDescriptor(rawValue: fd), try component(name))) }
}

func fdOpsOpenDirectory(fd: Int32, name: RustStr) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.openDirectory(FileDescriptor(rawValue: fd), try component(name)).rawValue }
}

func fdOpsMakeDirectory(fd: Int32, name: RustStr, hasPermissions: Bool, permissions: UInt16) -> CzOutcome {
  CzOutcome {
    try FileDescriptorOps.makeDirectory(
      FileDescriptor(rawValue: fd),
      try component(name),
      permissions: lentPermissions(hasPermissions, permissions)
    )
  }
}

func fdOpsUnlink(fd: Int32, name: RustStr) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.unlink(FileDescriptor(rawValue: fd), try component(name)) }
}

func fdOpsStatusOf(fd: Int32) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.status(of: FileDescriptor(rawValue: fd)) }
}

func fdOpsStatus(fd: Int32, name: RustStr) -> CzOutcome {
  CzOutcome { absent(try FileDescriptorOps.status(FileDescriptor(rawValue: fd), try component(name))) }
}

func fdOpsStatusRelativePath(fd: Int32, relativePath: RustStr, symlinks: FdOpsSymlinkPolicy) -> CzOutcome {
  let relativePath = FilePath(relativePath.toString())

  return CzOutcome {
    absent(
      try FileDescriptorOps.status(
        FileDescriptor(rawValue: fd),
        relativePath: relativePath,
        symlinks: symlinks.policy
      )
    )
  }
}

func fdOpsOpenFile(fd: Int32, name: RustStr) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.openFile(FileDescriptor(rawValue: fd), try component(name)).rawValue }
}

func fdOpsOpenFileRelativePath(fd: Int32, relativePath: RustStr, symlinks: FdOpsSymlinkPolicy) -> CzOutcome {
  let relativePath = FilePath(relativePath.toString())

  return CzOutcome {
    try FileDescriptorOps.openFile(
      FileDescriptor(rawValue: fd),
      relativePath: relativePath,
      symlinks: symlinks.policy
    ).rawValue
  }
}

func fdOpsCreateFile(fd: Int32, name: RustStr, hasPermissions: Bool, permissions: UInt16) -> CzOutcome {
  CzOutcome {
    try FileDescriptorOps.createFile(
      FileDescriptor(rawValue: fd),
      try component(name),
      permissions: lentPermissions(hasPermissions, permissions)
    ).rawValue
  }
}

func fdOpsMakeSymlink(fd: Int32, name: RustStr, target: RustStr) -> CzOutcome {
  let target = target.toString()

  return CzOutcome {
    try FileDescriptorOps.makeSymlink(FileDescriptor(rawValue: fd), try component(name), target: target)
  }
}

func fdOpsReadSymlink(fd: Int32, name: RustStr) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.readSymlink(FileDescriptor(rawValue: fd), try component(name)) }
}

func fdOpsUnlinkRecursive(fd: Int32, filename: RustStr) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.unlinkRecursive(FileDescriptor(rawValue: fd), filename: try component(filename)) }
}

/// `body` runs on this thread, once for each entry, and says whether it threw.
func fdOpsEnumerate(fd: Int32, body: RustEnumerate) -> CzOutcome {
  CzOutcome {
    try FileDescriptorOps.enumerate(FileDescriptor(rawValue: fd)) { path, type, parent in
      if body.call(path.string.intoRustString(), type.kind, parent.rawValue) {
        throw BridgeError.bodyFailed
      }
    }
  }
}

func fdOpsGetCanonicalPath(fd: Int32) -> CzOutcome {
  CzOutcome { try FileDescriptorOps.getCanonicalPath(FileDescriptor(rawValue: fd)).string }
}

/// `completion` runs on this thread, at most once, and says whether it threw.
func fdOpsMkdir(
  fd: Int32,
  relativePath: RustStr,
  hasPermissions: Bool,
  permissions: UInt16,
  makeIntermediates: Bool,
  completion: RustWithDescriptor
) -> CzOutcome {
  let relativePath = FilePath(relativePath.toString())

  return CzOutcome {
    try FileDescriptorOps.mkdir(
      FileDescriptor(rawValue: fd),
      relativePath,
      permissions: lentPermissions(hasPermissions, permissions),
      makeIntermediates: makeIntermediates
    ) { directory in
      if completion.call(directory.rawValue) {
        throw BridgeError.bodyFailed
      }
    }
  }
}

/// For a unit test: `FileDescriptorOps.maximumSymlinksFollowed`.
func fdOpsMaximumSymlinksFollowed() -> UInt {
  UInt(FileDescriptorOps.maximumSymlinksFollowed)
}

/// The one path component Rust passed as `name`.
private func component(_ name: RustStr) throws -> FilePath.Component {
  let name = name.toString()
  guard let component = FilePath.Component(name) else {
    throw BridgeError.malformed("path component", name)
  }
  return component
}

/// The `FilePermissions?` Rust passed as a flag and a raw value.
private func lentPermissions(_ hasPermissions: Bool, _ permissions: UInt16) -> FilePermissions? {
  hasPermissions ? FilePermissions(rawValue: permissions) : nil
}

extension FileDescriptorOps.EntryType {
  fileprivate var kind: FdOpsEntryType {
    switch self {
    case .regular: .Regular
    case .directory: .Directory
    case .symlink: .Symlink
    case .other: .Other
    }
  }
}

extension FdOpsSymlinkPolicy {
  fileprivate var policy: FileDescriptorOps.SymlinkPolicy {
    switch self {
    case .Refuse: .refuse
    case .FollowBeneath: .followBeneath
    }
  }
}

extension CzOutcome {
  func heldEntryType() -> FdOpsEntryType {
    (taken() as FileDescriptorOps.EntryType).kind
  }

  // A held `FileStatus`, field by field: Rust builds its own.

  func heldStatusType() -> FdOpsEntryType {
    (taken() as FileDescriptorOps.FileStatus).type.kind
  }

  func heldStatusPermissions() -> UInt16 {
    (taken() as FileDescriptorOps.FileStatus).permissions.rawValue
  }

  func heldStatusSize() -> Int64 {
    (taken() as FileDescriptorOps.FileStatus).size
  }

  func heldStatusUserId() -> UInt32 {
    (taken() as FileDescriptorOps.FileStatus).userID
  }

  func heldStatusGroupId() -> UInt32 {
    (taken() as FileDescriptorOps.FileStatus).groupID
  }

  func heldStatusModificationSeconds() -> Int64 {
    (taken() as FileDescriptorOps.FileStatus).modificationSeconds
  }

  func heldStatusModificationNanoseconds() -> Int64 {
    Int64((taken() as FileDescriptorOps.FileStatus).modificationNanoseconds)
  }

  // A held `FileDescriptorOps.Error`: its case, and what the case carries.

  func heldFdOpsErrorKind() -> FdOpsErrorKind {
    switch taken() as FileDescriptorOps.Error {
    case .invalidRelativePath: .InvalidRelativePath
    case .invalidPathComponent: .InvalidPathComponent
    case .cannotFollowSymlink: .CannotFollowSymlink
    case .notFound: .NotFound
    case .alreadyExists: .AlreadyExists
    case .conflict: .Conflict
    case .systemError: .SystemError
    }
  }

  /// A `conflict`'s entry type.
  func heldFdOpsErrorEntryType() -> FdOpsEntryType {
    guard case .conflict(let type) = taken() as FileDescriptorOps.Error else {
      preconditionFailure("Rust asks for the entry type only of a conflict")
    }
    return type.kind
  }

  /// A `systemError`'s operation.
  func heldFdOpsErrorOperation() -> String {
    guard case .systemError(let operation, _) = taken() as FileDescriptorOps.Error else {
      preconditionFailure("Rust asks for the operation only of a system error")
    }
    return operation
  }

  /// A `systemError`'s `errno`.
  func heldFdOpsErrorNumber() -> Int32 {
    guard case .systemError(_, let number) = taken() as FileDescriptorOps.Error else {
      preconditionFailure("Rust asks for the number only of a system error")
    }
    return number
  }
}
