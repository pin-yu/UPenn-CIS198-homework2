use std::path::PathBuf;

#[derive(Debug)]
pub struct MyFile {
  name: String,
  parent_dir: String,
  size_bytes: u64,
  depth: u32,
}

impl MyFile {
  pub fn from_path(path: PathBuf, depth: u32) -> MyFile {
    MyFile {
      name: String::from(path.file_name().unwrap().to_string_lossy()),
      parent_dir: String::from(path.parent().unwrap().to_string_lossy()),
      size_bytes: path.metadata().unwrap().len(),
      depth: depth,
    }
  }

  // implement regex and other filter function
}
