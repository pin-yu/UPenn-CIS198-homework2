mod file;

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use structopt::StructOpt;

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "rust_find", about = "A simple CLI to find files")]
struct Cli {
    #[structopt(short = "p", long, required = true)]
    pattern: String,

    #[structopt(short = "d", long, required = true)]
    dir: String,

    #[structopt(short = "o", long, required = true)]
    output: String,

    #[structopt(long)]
    depth: Option<u32>,

    #[structopt(long)]
    larger_than_mb: Option<u32>,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    println!("current executable: {:?}", env::current_exe());

    println!(
        "pattern:{}, dirs: {}, output: {}",
        args.pattern, args.dir, args.output
    );

    let mut result: Vec<file::MyFile> = Vec::new();

    visit_dir(&Path::new(&args.dir), &mut result, 1)?;

    for my_file in result {
        println!("{:?}", my_file);
    }

    Ok(())
}

fn visit_dir(dir: &Path, result: &mut Vec<file::MyFile>, depth: u32) -> io::Result<()> {
    if dir.is_dir() {
        // ? mark is a kind of error propagation mechanism in Rust
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dir(&path, result, depth + 1)?;
            } else {
                result.push(file::MyFile::from_path(path, depth));
            }
        }
    }
    Ok(())
}
