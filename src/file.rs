use crate::error::Error;
use std::{fmt::Display, fs::DirEntry, os::unix::fs::PermissionsExt};

pub struct File {
    pub name: String,
    pub file_type: FileType,
    pub is_hidden: bool,
    pub size: u64,
    pub permissions: Permissions,
}

impl File {
    pub fn get_human_redable_sizes(&self) -> String {
        let size = self.size;
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

    pub fn get_formatting(&self) -> &str {
        match self.file_type {
            FileType::File => "",
            FileType::Directory => "/",
            FileType::SymLink => "@",
        }
    }
}

impl TryFrom<DirEntry> for File {
    type Error = Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        let file_type: FileType = value
            .file_type()
            .map_err(|e| Error::UnableToReadFile(anyhow::anyhow!(e)))?
            .into();
        let name = value.file_name().into_string().map_err(|e| {
            Error::UnableToReadFile(if let Some(err) = e.to_str() {
                anyhow::anyhow!(err.to_owned())
            } else {
                anyhow::anyhow!("Unknown error")
            })
        })?;
        let metadata = value
            .metadata()
            .map_err(|e| Error::UnableToReadFile(anyhow::anyhow!(e)))?;

        Ok(File {
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
}

pub struct Permissions {
    pub permissions: u32,
    pub file_type: FileType,
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    SymLink,
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
