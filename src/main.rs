use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // TODO: add additional args from the man pages
    // -a: change only the access time
    // -c, --no-create: do not create any files
    // -d, --date=STRING: parse STRING and use it instead of current time
    // -f: (ignored)
    // -h, --no-deference: affect each symbolic link instead of any referenced file (only useful on
    // systems that can change the timestamp of a symlink)
    // -m: change only the modification time
    // -r, --reference=FILE: use this file's time instead of current time
    // -t STAMP: use [[CC]YY]MMDDhhmm[.ss] instead of current time
    // --time=WORD: change the specified time: WORD is access, atime, or use: equivalent to -a WORD
    // is modify or mtime: equivalent to -m
    //
    // Note that the -d and -t options accept different time-date formats.
    //
    // Date String:
    // The --date=STRING is a mostly free format human readable date string such as "Sun, 29 Feb
    // 2004 16:21:42 -0800" or "2004-02-29 16:21:42" or even "next Thursday". A date string may
    // contain items indicating calendar date, time of day, time zone, day of week, relative time,
    // relative date, and numbers. An empty string indicates the beginning of the day. The date
    // string format is more complex than is easily documented here but is fully described in the
    // info documentation.
    //
    // structopt docs: https://docs.rs/crate/structopt/0.3.13
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    // TODO: support args for multiple paths
}

fn main() {
    let args = Cli::from_args();
    let path = Path::new(&args.path);
    let display = path.display();

    if path.exists() {
        // TODO: update the access and modification times of each path to the current time
    } else {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    }
}
