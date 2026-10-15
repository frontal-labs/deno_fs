use deno_fs::FsFileType;
use deno_fs::OpenOptions;
use deno_fs::RealFs;

#[test]
fn open_options_map_access_modes_and_flags() {
  assert!(matches!(
    OpenOptions::read().access_kind(),
    deno_permissions::OpenAccessKind::Read
  ));
  assert!(matches!(
    OpenOptions::write(true, false, false, Some(0o644)).access_kind(),
    deno_permissions::OpenAccessKind::Write
  ));
  assert!(matches!(
    OpenOptions {
      read: true,
      create: true,
      ..Default::default()
    }
    .access_kind(),
    deno_permissions::OpenAccessKind::ReadWrite
  ));
}

#[test]
fn public_filesystem_types_are_constructible() {
  let _fs = RealFs;
  let entry = deno_fs::FsDirEntry {
    name: "entry".to_string(),
    is_file: true,
    is_directory: false,
    is_symlink: false,
  };
  assert!(entry.is_file);
  assert!(!matches!(FsFileType::Directory, FsFileType::File));
}
