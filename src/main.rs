use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use structopt::StructOpt;
use filetime::{FileTime, set_file_atime, set_file_mtime};

#[derive(StructOpt)]
struct Cli {
    // TODO: add additional args from the man pages
    // structopt docs: https://docs.rs/crate/structopt/0.3.13

    /// Change only the access time
    #[structopt(short = "a")]
    only_atime: bool,
    /// Do not create any files
    #[structopt(short = "c", long)]
    no_create: bool,
    /// (ignored)
    #[structopt(short = "f")]
    force: bool,
    /// Change only the modification time
    #[structopt(short = "m")]
    only_mtime: bool,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();

    for file in &args.files {
        let path = Path::new(&file);
        if path.exists() {
            if !&args.only_mtime {
                match set_file_atime(path, FileTime::now()) {
                    Err(e) => println!("{:?}", e),
                    _ => ()
                }
            }

            if !&args.only_atime {
                match set_file_mtime(path, FileTime::now()) {
                    Err(e) => println!("{:?}", e),
                    _ => ()
                }
            }
        } else if !&args.no_create {
            match File::create(&path) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
        }
    }
}
