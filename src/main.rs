use std::fmt::Display;
use std::fs::{self, DirEntry};
use std::io::Error;
use std::os::unix::fs::{MetadataExt as OtherMetadataExt, PermissionsExt};

const ERROR_LISTING_FILES: &str = "there was an error listing files";

#[derive(Default)]
struct Options {
    all: bool,
    f: bool,
    l: bool,
    h: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileType {
    File,
    Directory,
    SymLink,
}

struct File {
    name: String,
    file_type: FileType,
    is_hidden: bool,
    size: u64,
    permissions: Permissions,
}

struct Permissions {
    permissions: u32,
    file_type: FileType,
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::with_capacity(11);
        match self.file_type {
            FileType::File => result.push('-'),
            FileType::Directory => result.push('d'),
            FileType::SymLink => result.push('l'),
        }

        let mut i = 8;
        while i >= 0 {
            if self.permissions >> i & 0x1 == 1 {
                if i == 0 || i == 3 || i == 6 {
                    result.push('x');
                } else if i == 1 || i == 4 || i == 7 {
                    result.push('w');
                } else {
                    result.push('r');
                }
            } else {
                result.push('-');
            }

            i -= 1;
        }

        if cfg!(target_os = "macos") {
            result.push('@');
        }

        write!(f, "{result}")
    }
}

impl From<std::fs::FileType> for FileType {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_dir() {
            FileType::Directory
        } else if value.is_symlink() {
            FileType::SymLink
        } else {
            FileType::File
        }
    }
}

fn get_human_redable_sizes(size: u64) -> String {
    if size < 1024 {
        format!("{size}B")
    } else if (1024..1024 * 1024).contains(&size) {
        format!("{}K", size / 1024)
    } else if (1024 * 1024..1024 * 1024 * 1024).contains(&size) {
        format!("{}M", size / (1024 * 1024))
    } else if (1024 * 1024 * 2014..1024 * 1024 * 1024 * 1024).contains(&size) {
        format!("{}G", size / (1024 * 1024 * 1024))
    } else {
        format!("{}T", size / (1024 * 1024))
    }
}

fn get_formatting(file: &File) -> &str {
    match file.file_type {
        FileType::File => "",
        FileType::Directory => "/",
        FileType::SymLink => "@",
    }
}

fn parse_file(dir_entry: DirEntry) -> Option<File> {
    let file_type: FileType = dir_entry.file_type().ok()?.into();
    let name = dir_entry.file_name().into_string().ok()?;
    let metadata = dir_entry.metadata().ok()?;

    Some(File {
        is_hidden: name.starts_with('.'),
        name,
        file_type,
        size: metadata.len(),
        permissions: Permissions {
            permissions: metadata.permissions().mode(),
            file_type,
        },
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut options = Options::default();

    for arg in &args {
        if arg == "-a" || arg == "--all" {
            options.all = true;
        }

        if arg == "-F" {
            options.f = true;
        }

        if arg == "-h" {
            options.h = true;
        }

        if arg == "-l" {
            options.l = true;
        }
    }

    let path = args.last().map(|s| s.as_str()).unwrap_or(".");
    let paths = fs::read_dir(path).expect(ERROR_LISTING_FILES);

    let mut dir_entries: Vec<File> = paths
        .into_iter()
        .collect::<Result<Vec<DirEntry>, Error>>()
        .expect(ERROR_LISTING_FILES)
        .into_iter()
        .filter_map(parse_file)
        .collect();

    dir_entries.sort_by(|f1, f2| {
        if f1.name < f2.name {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let hidden_files: Vec<&File> = dir_entries.iter().filter(|s| s.is_hidden).collect();
    let unhidden_files: Vec<&File> = dir_entries.iter().filter(|s| !s.is_hidden).collect();

    let current_dir = std::fs::File::open(".")
        .expect("unable to get information for .")
        .metadata()
        .expect("unable to get information for .");

    let mut result: Vec<&File> = vec![];
    let current_dir = File {
        name: ".".to_owned(),
        file_type: FileType::Directory,
        is_hidden: true,
        size: current_dir.size(),
        permissions: Permissions {
            permissions: current_dir.permissions().mode(),
            file_type: FileType::Directory,
        },
    };

    let before_dir = std::fs::File::open("..")
        .expect("unable to get information for .")
        .metadata()
        .expect("unable to get information for .");

    let before_dir = File {
        name: "..".to_owned(),
        file_type: FileType::Directory,
        is_hidden: true,
        size: before_dir.size(),
        permissions: Permissions {
            permissions: before_dir.permissions().mode(),
            file_type: FileType::Directory,
        },
    };

    if options.all {
        result.push(&current_dir);
        result.push(&before_dir);
        result.extend(hidden_files);
    }

    result.extend(unhidden_files);

    for (i, f) in result.iter().enumerate() {
        let format = if options.f { get_formatting(f) } else { "" };
        let filename = format!("{}{format}", f.name);
        let size = if options.h {
            get_human_redable_sizes(f.size)
        } else {
            format!("{}", f.size)
        };

        if options.l {
            println!("{}\t{size}\t{filename}", f.permissions);
        } else {
            if i != 0 {
                print!("\t");
            }
            print!("{filename}");

            if i == result.len() - 1 {
                println!();
            }
        }
    }
}
