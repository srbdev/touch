use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::fs::metadata;
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
    /// Use this file's times instead of current time
    #[structopt(short, long, parse(from_os_str))]
    reference: Option<PathBuf>,
    /// Change only the modification time
    #[structopt(short = "m")]
    only_mtime: bool,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let mut atime = FileTime::now();
    let mut mtime = FileTime::now();

    let ref_path = match args.reference {
        Some(p) => p,
        None => PathBuf::new(),
    };

    if ref_path.file_name() != None {
        let rp = Path::new(&ref_path); // rp is local for ref_path
        if rp.exists() {
            let meta = metadata(rp).unwrap();  // TODO make this better...?
            atime = FileTime::from_last_access_time(&meta);
            mtime = FileTime::from_last_modification_time(&meta);
        }
    }

    for file in &args.files {
        let path = Path::new(&file);
        if path.exists() {
            if !&args.only_mtime {
                match set_file_atime(path, atime) {
                    Err(e) => println!("{:?}", e),
                    _ => ()
                }
            }

            if !&args.only_atime {
                match set_file_mtime(path, mtime) {
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
