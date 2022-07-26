mod file;

use std::env;
use std::io;

use structopt::StructOpt;

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
#[structopt(name = "rust_find", about = "A simple CLI to find files")]
struct Cli {
    #[structopt(short = "p", long, required = true)]
    pattern: String,

    #[structopt(short = "d", long, required = true)]
    dir: String,

    #[structopt(short = "o", long)]
    output: Option<String>,

    #[structopt(long)]
    depth: Option<u32>,

    #[structopt(long)]
    larger_than_mb: Option<u64>,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    println!("current executable: {:?}", env::current_exe());

    match &args.output {
        Some(output) => println!(
            "pattern:{}, dir: {}, output: {}",
            args.pattern, args.dir, output
        ),
        None => println!("pattern:{}, dir: {}", args.pattern, args.dir),
    }

    let mut my_files = file::MyFiles::from_dir(&args.dir);
    my_files.retain_match_reg_pattern(&args.pattern);

    match args.depth {
        Some(depth) => my_files.retain_depth_less_than(depth),
        None => (),
    }

    match args.larger_than_mb {
        Some(size_in_mb) => my_files.retain_size_larger_than(size_in_mb),
        None => (),
    }

    my_files.output(args.output)
}
