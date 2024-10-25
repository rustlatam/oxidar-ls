mod cli_arguments;
mod error;
mod file;

use std::fs::{self, DirEntry};
use std::os::unix::fs::{MetadataExt as OtherMetadataExt, PermissionsExt};
use std::process::ExitCode;

use anyhow::anyhow;
use clap::Parser;
use cli_arguments::CliArguments;
use error::Error;
use file::{File, FileType, Permissions};

fn main() -> ExitCode {
    let args = CliArguments::parse();

    if let Err(err) = list_files(&args) {
        eprintln!("{err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn list_files(args: &CliArguments) -> Result<(), Error> {
    let paths = fs::read_dir(&args.path).map_err(|e| Error::ErrorListingFiles(anyhow!(e)))?;

    let mut dir_entries = paths
        .into_iter()
        .collect::<Result<Vec<DirEntry>, std::io::Error>>()
        .map_err(|e| Error::ErrorListingFiles(anyhow!(e)))?
        .into_iter()
        .map(File::try_from)
        .collect::<Result<Vec<File>, Error>>()
        .map_err(|e| Error::ErrorListingFiles(anyhow!(e)))?;

    dir_entries.sort_by(|f1, f2| {
        if f1.name < f2.name {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let hidden_files: Vec<&File> = dir_entries.iter().filter(|s| s.is_hidden).collect();
    let unhidden_files: Vec<&File> = dir_entries.iter().filter(|s| !s.is_hidden).collect();

    let mut result: Vec<&File> = vec![];

    let current_dir = get_file_information(".")?;
    let before_dir = get_file_information("..")?;
    if args.all {
        result.push(&current_dir);
        result.push(&before_dir);
        result.extend(hidden_files);
    }

    result.extend(unhidden_files);

    for (i, f) in result.iter().enumerate() {
        let format = if args.format { f.get_formatting() } else { "" };
        let filename = format!("{}{format}", f.name);
        let size = if args.human {
            f.get_human_redable_sizes()
        } else {
            format!("{}", f.size)
        };

        if args.list {
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

    Ok(())
}

/// Gets a file information manually
fn get_file_information(path: &str) -> Result<File, Error> {
    let current_dir = std::fs::File::open(path)
        .map_err(|e| Error::UnableToReadFile(anyhow!(e)))?
        .metadata()
        .map_err(|e| Error::UnableToReadFile(anyhow!(e)))?;

    Ok(File {
        name: path.to_owned(),
        file_type: FileType::Directory,
        is_hidden: true,
        size: current_dir.size(),
        permissions: Permissions {
            permissions: current_dir.permissions().mode(),
            file_type: FileType::Directory,
        },
    })
}
