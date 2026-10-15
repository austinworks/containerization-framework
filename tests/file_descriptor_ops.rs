#![cfg(feature = "integration")]

//! ContainerizationOS: `FileDescriptorOps`.

use containerization_framework as cfw;
use std::ffi::OsStr;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsFd;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

/// A temporary directory, and a descriptor for it.
fn directory() -> (tempfile::TempDir, std::fs::File) {
  let directory = tempfile::tempdir().expect("a temporary directory");
  let descriptor = std::fs::File::open(directory.path()).expect("the directory should open");

  (directory, descriptor)
}

/// What `FileDescriptorOps` threw, from an error that should carry it.
fn thrown(error: cfw::Error) -> cfw::containerization_os::file_descriptor_ops::Error {
  match error {
    cfw::Error::FileDescriptorOps { error, .. } => error,
    other => panic!("FileDescriptorOps should have thrown, not {other:?}"),
  }
}

#[test]
fn creates_reads_and_removes_entries_beneath_a_directory() {
  let (directory, root) = directory();

  cfw::containerization_os::file_descriptor_ops::make_directory(root.as_fd(), OsStr::new("sub"), None)
    .expect("the directory should be made");
  let mut file = std::fs::File::from(
    cfw::containerization_os::file_descriptor_ops::create_file(root.as_fd(), OsStr::new("file.txt"), Some(0o600))
      .expect("the file should be created"),
  );
  file.write_all(b"written").expect("the file should write");
  drop(file);
  cfw::containerization_os::file_descriptor_ops::make_symlink(root.as_fd(), OsStr::new("link"), "file.txt")
    .expect("the symlink should be made");

  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::entry_type(root.as_fd(), OsStr::new("sub"))
      .expect("the type of sub"),
    Some(cfw::containerization_os::file_descriptor_ops::EntryType::Directory)
  );
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::entry_type(root.as_fd(), OsStr::new("link"))
      .expect("the type of link"),
    Some(cfw::containerization_os::file_descriptor_ops::EntryType::Symlink),
    "a symlink should not be followed"
  );
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::entry_type(root.as_fd(), OsStr::new("missing"))
      .expect("the type of a missing entry"),
    None
  );
  let status = cfw::containerization_os::file_descriptor_ops::status(root.as_fd(), OsStr::new("file.txt"))
    .expect("the file's status")
    .expect("the file exists");
  assert_eq!(
    status.r#type,
    cfw::containerization_os::file_descriptor_ops::EntryType::Regular
  );
  assert_eq!(status.permissions, 0o600);
  assert_eq!(status.size, 7);
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::read_symlink(root.as_fd(), OsStr::new("link"))
      .expect("the link's target"),
    "file.txt"
  );

  let mut contents = String::new();
  std::fs::File::from(
    cfw::containerization_os::file_descriptor_ops::open_file(root.as_fd(), OsStr::new("file.txt"))
      .expect("the file should open"),
  )
  .read_to_string(&mut contents)
  .expect("the file should read");
  assert_eq!(contents, "written");

  let sub = cfw::containerization_os::file_descriptor_ops::open_directory(root.as_fd(), OsStr::new("sub"))
    .expect("the directory should open");
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::status_of(sub.as_fd())
      .expect("the directory's status")
      .r#type,
    cfw::containerization_os::file_descriptor_ops::EntryType::Directory
  );
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::get_canonical_path(sub.as_fd()).expect("the directory's path"),
    directory
      .path()
      .canonicalize()
      .expect("the canonical temporary directory")
      .join("sub")
  );
  drop(
    cfw::containerization_os::file_descriptor_ops::create_file(sub.as_fd(), OsStr::new("nested.txt"), None)
      .expect("a file should be created in the opened directory"),
  );

  cfw::containerization_os::file_descriptor_ops::unlink(root.as_fd(), OsStr::new("link"))
    .expect("the link should unlink");
  cfw::containerization_os::file_descriptor_ops::unlink_recursive(root.as_fd(), OsStr::new("sub"))
    .expect("the directory should be removed");
  assert!(
    directory.path().join("file.txt").exists(),
    "unlinking the link should keep its target"
  );
  assert!(!directory.path().join("link").exists());
  assert!(!directory.path().join("sub").exists());
}

#[test]
fn says_what_is_in_the_way() {
  let (directory, root) = directory();
  std::fs::create_dir(directory.path().join("sub")).expect("a directory");
  std::fs::write(directory.path().join("file.txt"), "").expect("a file");
  std::os::unix::fs::symlink("file.txt", directory.path().join("link")).expect("a symlink");

  let open_file = |name: &str| cfw::containerization_os::file_descriptor_ops::open_file(root.as_fd(), OsStr::new(name));

  assert_eq!(
    thrown(open_file("link").expect_err("a symlink should not be followed")),
    cfw::containerization_os::file_descriptor_ops::Error::CannotFollowSymlink
  );
  assert_eq!(
    thrown(open_file("sub").expect_err("a directory is not a file")),
    cfw::containerization_os::file_descriptor_ops::Error::Conflict(
      cfw::containerization_os::file_descriptor_ops::EntryType::Directory
    )
  );
  assert_eq!(
    thrown(open_file("missing").expect_err("a missing file should not open")),
    cfw::containerization_os::file_descriptor_ops::Error::NotFound
  );
  assert_eq!(
    thrown(
      cfw::containerization_os::file_descriptor_ops::create_file(root.as_fd(), OsStr::new("file.txt"), None)
        .expect_err("an existing file should not be created again")
    ),
    cfw::containerization_os::file_descriptor_ops::Error::AlreadyExists
  );

  let error = open_file("sub/file.txt").expect_err("a name is one component");
  assert!(matches!(error, cfw::Error::Failed { .. }), "{error:?}");
}

#[test]
fn follows_a_symlink_only_beneath_its_directory_when_told() {
  let (directory, root) = directory();
  std::fs::create_dir(directory.path().join("sub")).expect("a directory");
  std::fs::write(directory.path().join("sub").join("file.txt"), "beneath").expect("a file");
  std::os::unix::fs::symlink("sub/file.txt", directory.path().join("link")).expect("a symlink");
  std::os::unix::fs::symlink("../outside.txt", directory.path().join("escape")).expect("a symlink");

  let open = |path: &str, symlinks| {
    cfw::containerization_os::file_descriptor_ops::open_file_relative_path(root.as_fd(), Path::new(path), symlinks)
  };

  assert_eq!(
    thrown(
      open(
        "link",
        cfw::containerization_os::file_descriptor_ops::SymlinkPolicy::Refuse
      )
      .expect_err("a refused symlink should not be followed")
    ),
    cfw::containerization_os::file_descriptor_ops::Error::CannotFollowSymlink
  );
  let mut contents = String::new();
  std::fs::File::from(
    open(
      "link",
      cfw::containerization_os::file_descriptor_ops::SymlinkPolicy::FollowBeneath,
    )
    .expect("a symlink beneath the directory should be followed"),
  )
  .read_to_string(&mut contents)
  .expect("the file should read");
  assert_eq!(contents, "beneath");
  assert_eq!(
    thrown(
      open(
        "escape",
        cfw::containerization_os::file_descriptor_ops::SymlinkPolicy::FollowBeneath
      )
      .expect_err("a symlink out of the directory should not be followed")
    ),
    cfw::containerization_os::file_descriptor_ops::Error::CannotFollowSymlink
  );
  assert_eq!(
    thrown(
      open(
        "../outside.txt",
        cfw::containerization_os::file_descriptor_ops::SymlinkPolicy::FollowBeneath
      )
      .expect_err("a path with .. should be refused")
    ),
    cfw::containerization_os::file_descriptor_ops::Error::InvalidRelativePath
  );

  let status = |path: &str| {
    cfw::containerization_os::file_descriptor_ops::status_relative_path(
      root.as_fd(),
      Path::new(path),
      cfw::containerization_os::file_descriptor_ops::SymlinkPolicy::Refuse,
    )
  };
  assert_eq!(
    status("sub/file.txt")
      .expect("the file's status")
      .map(|status| status.size),
    Some(7)
  );
  assert_eq!(status("sub/missing.txt").expect("a missing file's status"), None);
}

#[test]
fn makes_a_path_of_directories_and_works_inside_it() {
  let (directory, root) = directory();

  cfw::containerization_os::file_descriptor_ops::mkdir(
    root.as_fd(),
    Path::new("a/b/c"),
    cfw::containerization_os::file_descriptor_ops::MkdirOptions {
      make_intermediates: true,
      ..Default::default()
    },
    |made| {
      drop(cfw::containerization_os::file_descriptor_ops::create_file(
        made,
        OsStr::new("inside.txt"),
        None,
      )?);
      Ok(())
    },
  )
  .expect("the directories should be made");
  assert!(directory.path().join("a/b/c/inside.txt").is_file());

  assert_eq!(
    thrown(
      cfw::containerization_os::file_descriptor_ops::mkdir(root.as_fd(), Path::new("x/y"), Default::default(), |_| Ok(
        ()
      ))
      .expect_err("a missing parent should not be made by default")
    ),
    cfw::containerization_os::file_descriptor_ops::Error::InvalidPathComponent
  );

  let returned = cfw::Error::failed("work inside", "the completion failed");
  let thrown_by_completion = returned.clone();
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::mkdir(root.as_fd(), Path::new("z"), Default::default(), move |_| {
      Err(thrown_by_completion)
    })
    .expect_err("the completion's error should come back"),
    returned
  );
}

#[test]
fn enumerates_a_tree_without_following_symlinks() {
  let (directory, root) = directory();
  std::fs::create_dir(directory.path().join("sub")).expect("a directory");
  std::fs::write(directory.path().join("sub").join("file.txt"), "").expect("a file");
  std::os::unix::fs::symlink("sub", directory.path().join("link")).expect("a symlink");

  let entries = Arc::new(Mutex::new(Vec::new()));
  let seen = Arc::clone(&entries);
  cfw::containerization_os::file_descriptor_ops::enumerate(root.as_fd(), move |path, entry_type, parent| {
    assert!(
      cfw::containerization_os::file_descriptor_ops::status_of(parent).is_ok(),
      "the parent should be open during the call"
    );
    seen
      .lock()
      .expect("the entries")
      .push((path.to_path_buf(), entry_type));
    Ok(())
  })
  .expect("the tree should enumerate");

  let mut entries = entries.lock().expect("the entries").clone();
  entries.sort_by(|(left, _), (right, _)| left.cmp(right));
  assert_eq!(
    entries,
    [
      (
        PathBuf::from("link"),
        cfw::containerization_os::file_descriptor_ops::EntryType::Symlink
      ),
      (
        PathBuf::from("sub"),
        cfw::containerization_os::file_descriptor_ops::EntryType::Directory
      ),
      (
        PathBuf::from("sub/file.txt"),
        cfw::containerization_os::file_descriptor_ops::EntryType::Regular
      ),
    ]
  );

  // Swift reads the directory through a duplicate, which shares the
  // descriptor's offset, so a second walk needs a descriptor of its own.
  let reopened = std::fs::File::open(directory.path()).expect("the directory should open again");
  let returned = cfw::Error::failed("enumerate", "the body failed");
  let thrown_by_body = returned.clone();
  assert_eq!(
    cfw::containerization_os::file_descriptor_ops::enumerate(reopened.as_fd(), move |_, _, _| Err(
      thrown_by_body.clone()
    ))
    .expect_err("the body's error should come back"),
    returned
  );
}
