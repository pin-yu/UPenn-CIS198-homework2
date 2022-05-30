struct MyFile {
  name: String,
  dir_in: String,
  size_bytes: u64,
}

impl MyFile {
  fn from_path(path: PathBuf) -> Reulst<Self> {
    let metadata = fs::metadata(path)?;
    metadata.created().unwrap();
  }
}
