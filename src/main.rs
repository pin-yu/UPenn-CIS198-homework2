use std::env;
use std::fs;
use std::io;

use structopt::StructOpt;

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "rust_find", about = "A simple CLI to find files")]
struct Cli {
    #[structopt(short = "p", long, required = true)]
    pattern: String,

    #[structopt(short = "d", long, required = true)]
    dirs: String,

    #[structopt(short = "o", long, required = true)]
    output: String,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    println!("current executable: {:?}", env::current_exe());

    println!(
        "pattern:{}, dirs: {}, output: {}",
        args.pattern, args.dirs, args.output
    );

    // ? mark is a kind of error propagation mechanism in Rust
    for entry in fs::read_dir(args.dirs)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.to_string_lossy());
    }

    Ok(())
}
