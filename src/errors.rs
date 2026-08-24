//! # Обработка ошибок для утилиты `tree`
//!
//! Модуль для обработки и отображения ошибок, возникающих
//! при работе с файловой системой и аргументами командной строки

use std::path::PathBuf;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum TreeErrors {
    NotFound(PathBuf),
    NotADirectory(PathBuf),
    InvalidDepth(String),
    UnknownFlag(String),
    MissingExtensionValue,
    Io(io::Error),
}

impl fmt::Display for TreeErrors {
    fn fmt(&self, buffer: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreeErrors::NotFound(path) => {
                write!(buffer, "Error: path '{}' does not exist!", path.display())
            }
            TreeErrors::NotADirectory(path) => {
                write!(buffer, "Error: '{}' is not a directory!", path.display())
            }
            TreeErrors::InvalidDepth(val) => {
                write!(buffer, "Error: invalid depth value: '{}' | Expected a positive integer!", val)
            }
            TreeErrors::UnknownFlag(flag) => {
                write!(buffer, "Error: unknown flag '{}'! Use -h or --help for help", flag)
            }
            TreeErrors::Io(err) => {
                write!(buffer, "I/O error: {}", err)
            }
            TreeErrors::MissingExtensionValue => {
                write!(buffer, "Error: missing extension value for -e / --ext flag!")
            }
        }
    }
}

impl From<io::Error> for TreeErrors {
    fn from(err: io::Error) -> Self {
        TreeErrors::Io(err)
    }
}
