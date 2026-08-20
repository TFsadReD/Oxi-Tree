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
    Io(io::Error),
}

impl fmt::Display for TreeErrors {
    fn fmt(&self, buffer: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreeErrors::NotFound(path) => {
                write!(buffer, "Ошибка: путь '{}' не существует!", path.display())
            }
            TreeErrors::NotADirectory(path) => {
                write!(buffer, "Ошибка: '{}' не является директорией!", path.display())
            }
            TreeErrors::InvalidDepth(val) => {
                write!(buffer, "Ошибка: некорректное значение глубины: '{}' | Ожидается целое положительное число!", val)
            }
            TreeErrors::UnknownFlag(flag) => {
                write!(buffer, "Ошибка: неизвестный флаг '{}'. Используйте -h или --help для справки.", flag)
            }
            TreeErrors::Io(err) => {
                write!(buffer, "Ошибка ввода-вывода: {}", err)
            }
        }
    }
}

impl From<io::Error> for TreeErrors {
    fn from(err: io::Error) -> Self {
        TreeErrors::Io(err)
    }
}
