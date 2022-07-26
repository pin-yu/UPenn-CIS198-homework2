use regex::Regex;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct MyFiles {
    files: Vec<MyFile>,
}

#[derive(Debug)]
pub struct MyFile {
    path_buf: PathBuf,
    size_bytes: u64,
    depth: u32,
}

impl MyFiles {
    pub fn from_dir(dir_str: &str) -> MyFiles {
        let dir = Path::new(dir_str);
        let init_depth = 1;
        let mut files: Vec<MyFile> = Vec::new();

        visit_dir(dir, init_depth, &mut files).unwrap();

        MyFiles { files }
    }

    pub fn retain_match_reg_pattern(&mut self, pattern: &str) {
        match Regex::new(pattern) {
            Ok(re) => self
                .files
                .retain(|file| re.is_match(file.path_buf.to_str().unwrap())),
            _ => println!("bad regular expression"),
        }
    }

    pub fn retain_depth_less_than(&mut self, depth: u32) {
        self.files.retain(|file| file.depth <= depth)
    }

    pub fn retain_size_larger_than(&mut self, size_in_mb: u64) {
        let size_in_byte = size_in_mb * (2 << 19);
        // println!("size_in_mb: {}", size_in_mb);
        // println!("filter size: {}", size_in_byte);
        self.files.retain(|file| file.size_bytes > size_in_byte)
    }

    pub fn output(&self, output_dir: Option<String>) -> std::io::Result<()> {
        match output_dir {
            Some(output) => {
                let mut new_file = fs::File::create(output)?;
                new_file.write_all(self.to_string().as_bytes())?;
                Ok(())
            }
            None => {
                println!("{}", self);
                Ok(())
            }
        }
    }
}

impl fmt::Display for MyFiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.files
                .iter()
                .map(|file| file.path_buf.to_str().unwrap().to_owned() + "\n")
                .collect::<String>()
        )
    }
}

impl MyFile {
    pub fn from_path(path: PathBuf, depth: u32) -> MyFile {
        MyFile {
            size_bytes: path.metadata().unwrap().len(),
            depth,
            path_buf: path,
        }
    }

    // implement regex and other filter function
}

fn visit_dir(dir: &Path, depth: u32, files: &mut Vec<MyFile>) -> io::Result<()> {
    if dir.is_dir() {
        // ? mark is an error propagation mechanism in Rust
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path, depth + 1, files)?;
            } else {
                files.push(MyFile::from_path(path, depth));
            }
        }
    }
    Ok(())
}
