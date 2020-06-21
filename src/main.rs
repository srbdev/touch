use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::fs::metadata;
use structopt::StructOpt;
use filetime::{FileTime, set_file_atime, set_file_mtime};
use std::str::FromStr;

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
    // /// (ignored)
    // #[structopt(short = "f")]
    // force: bool,
    /// Use this file's times instead of current time
    #[structopt(short, long, parse(from_os_str))]
    reference: Option<PathBuf>,
    /// Change only the modification time
    #[structopt(short = "m")]
    only_mtime: bool,
    /// Use [[CC]YY]MMDDhhmm[.ss] instead of current time
    #[structopt(short = "t")]
    time_stamp: Option<String>,
    /// Change the specified time: Word is access, atime, or use: equivalent to `-a` Word is modify
    /// or mtime: equivalent to `-m`
    #[structopt(long)]
    time: Option<String>,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn parse_seconds(stamp: &String) -> u8 {
    let tokens: Vec<&str> = stamp.split(".").collect();
    let mut secs_u8 = 0;

    if tokens.len() > 1 {
        let mut secs= tokens[1].to_string();

        if secs.len() == 1 {
            secs.push_str("0");
        }

        secs_u8 = match u8::from_str(secs.as_str()) {
            Ok(s) => s,
            Err(_) => 0,
        };

        if secs_u8 > 59 {
            secs_u8 = 0;
        }
    }

    return secs_u8;
}

fn parse_minutes(stamp: &String) -> u8 {
    let tokens: Vec<&str> = stamp.split(".").collect();
    let mins = tokens[0];
    let mut mins_u8 = 0;

    if let Some((i, _)) = mins.char_indices().rev().nth(1) {
        let mins_str = &mins[i..];

        mins_u8 = match u8::from_str(mins_str) {
            Ok(s) => s,
            Err(_) => 0,
        };

        if mins_u8 > 59 {
            mins_u8 = 0;
        }
    }

    return mins_u8;
}

fn parse_hours(_stamp: &String) -> u8 {
    return 0;
}

fn parse_day(_stamp: &String) -> u8 {
    return 1;
}

fn parse_month(_stamp: &String) -> u8 {
    return 1;
}

fn parse_year(_stamp: &String) -> u16 {
    return 2020;
}

pub fn parse_tstamp(stamp: &String) -> FileTime {
    let _year = parse_year(&stamp);
    let _month = parse_month(&stamp);
    let _day = parse_day(&stamp);
    let _hour = parse_hours(&stamp);
    let _minutes = parse_minutes(&stamp);
    let _seconds = parse_seconds(&stamp);

    return FileTime::now();
}


fn main() {
    let args = Cli::from_args();
    let mut only_atime = false;
    let mut only_mtime = false;

    let tstamp = match args.time_stamp {
        Some(s) => parse_tstamp(&s),
        None => FileTime::now(),
    };
    let mut atime = tstamp;
    let mut mtime = tstamp;

    let ref_path = match args.reference {
        Some(p) => p,
        None => PathBuf::new(),
    };

    let time = match &args.time {
        Some(t) => t,
        None => "",
    };

    if time == "access" || time == "atime" || time == "use" || args.only_atime == true {
        only_atime = true;
    }

    if time == "modify" || time == "mtime" || args.only_mtime == true {
        only_mtime = true;
    }

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
            if only_mtime || (!only_mtime && !only_atime) {
                match set_file_atime(path, atime) {
                    Err(e) => println!("{:?}", e),
                    _ => ()
                }
            }

            if only_atime || (!only_mtime && !only_atime) {
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


#[cfg(test)]
mod tests {
    use super::*;

    // [[CC]YY]MMDDhhmm[.ss] for reference...
    #[test]
    fn test_parse_seconds() {
        assert_eq!(0, parse_seconds(&String::from("01010000")));
        assert_eq!(0, parse_seconds(&String::from("01010000.00")));
        assert_eq!(30, parse_seconds(&String::from("01010000.30")));
        assert_eq!(0, parse_seconds(&String::from("")));
        assert_eq!(7, parse_seconds(&String::from("01010000.07")));
        assert_eq!(5, parse_seconds(&String::from(".05")));
        assert_eq!(50, parse_seconds(&String::from("01010000.5")));
        assert_eq!(0, parse_seconds(&String::from("test")));
        assert_eq!(0, parse_seconds(&String::from("test.test")));
        assert_eq!(0,parse_seconds(&String::from("01010000.75")));
    }

    #[test]
    fn test_parse_minutes() {
        assert_eq!(0, parse_minutes(&String::from("01010000")));
        assert_eq!(0, parse_minutes(&String::from("01010000.00")));
        assert_eq!(1, parse_minutes(&String::from("01010001")));
        assert_eq!(34, parse_minutes(&String::from("01010034.00")));
        assert_eq!(0, parse_minutes(&String::from("01010060")));
        assert_eq!(0, parse_minutes(&String::from("test")));
        assert_eq!(0, parse_minutes(&String::from("test.test")));
        assert_eq!(0, parse_minutes(&String::from("")));
    }
}
