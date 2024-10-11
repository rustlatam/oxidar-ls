use std::fs::{self, DirEntry};
use std::io::Error;

const ERROR_LISTING_FILES: &str = "there was an error listing files";

#[derive(Default)]
struct Options {
    all: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut options = Options::default();

    for arg in &args {
        if arg == "-a" || arg == "--all" {
            options.all = true;
        }
    }

    let path = args.last().map(|s| s.as_str()).unwrap_or(".");
    let paths = fs::read_dir(path).expect(ERROR_LISTING_FILES);

    let mut dir_entries: Vec<String> = paths
        .into_iter()
        .collect::<Result<Vec<DirEntry>, Error>>()
        .expect(ERROR_LISTING_FILES)
        .into_iter()
        .filter_map(|f| f.file_name().into_string().ok())
        .collect();

    dir_entries.sort();

    let hidden_files: Vec<&String> = dir_entries.iter().filter(|s| s.starts_with('.')).collect();
    let unhidden_files: Vec<&String> = dir_entries.iter().filter(|s| !s.starts_with('.')).collect();

    if options.all {
        print!(".\t..\t");
        for file in hidden_files {
            print!("{}\t", file)
        }
    }

    for file in unhidden_files {
        print!("{}\t", file)
    }
    println!();
}
