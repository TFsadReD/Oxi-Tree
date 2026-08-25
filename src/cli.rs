//! # Обработка командной строки для утилиты `tree`
//!
//! Модуль для обработки аргументов командной строки, флагов,
//! путей и других параметров, передаваемых пользователем

use crate::errors::TreeErrors;

use std::env;
use std::path::PathBuf;

/// Максимальная глубина рекурсивного погружения по умолчанию
const DEFAULT_MAX_DEPTH: usize = 2;

pub struct Args {
    pub path: PathBuf,
    pub depth: usize,
    pub show_hidden: bool,
    pub show_help: bool,
    pub dirs_only: bool,
    pub show_version: bool,
    pub unsorted: bool,
    pub ext: Option<String>,
}

/// Парсит аргументы командной строки для настройки параметров утилиты
/// Функция обрабатывает флаги и пути, переданные пользователем, и возвращает
/// структуру с настроенными параметрами, такими как путь и глубина обхода
pub fn parse_arg() -> Result<Args, TreeErrors> {
    let mut path = PathBuf::from(".");
    let mut depth = DEFAULT_MAX_DEPTH;
    let mut show_hidden = false;
    let mut show_help = false;
    let mut dirs_only = false;
    let mut show_version = false;
    let mut unsorted = false;
    let mut ext = None;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--depth" => {
                depth = parse_depth(&args, i)?;
                i += 2;
                continue;
            }

            "-e" | "--ext" => {
                ext = Some(parse_ext(&args, i)?);
                i += 2;
                continue;
            }

            "-nd" | "--no-depth" => {
                depth = usize::MAX;
            }

            "-a" | "--all" => {
                show_hidden = true;
            }

            "-h" | "--help" => {
                show_help = true;
            }

            "-D" | "--dirs-only" => {
                dirs_only = true;
            }

            "-v" | "--version" => {
                show_version = true;
            }

            "-U" | "--unsorted" => {
                unsorted = true;
            }

            flag if flag.starts_with('-') => {
                return Err(TreeErrors::UnknownFlag(flag.to_string()));
            }

            _ => {
                path = PathBuf::from(&args[i]);
            }
        }

        i += 1;
    }

    Ok(Args {
        path,
        depth,
        show_hidden,
        show_help,
        dirs_only,
        show_version,
        unsorted,
        ext,
    })
}

/// Парсит значение глубины рекурсии из аргументов командной строки
fn parse_depth(args: &[String], current_index: usize) -> Result<usize, TreeErrors> {
    if current_index + 1 < args.len() {
        if let Ok(parsed_depth) = args[current_index + 1].parse::<usize>() {
            Ok(parsed_depth)
        } else {
            Err(TreeErrors::InvalidDepth(args[current_index + 1].clone()))
        }
    } else {
        Err(TreeErrors::InvalidDepth("значение отсутствует".to_string()))
    }
}

/// Парсит значение расширения файла из аргументов командной строки
fn parse_ext(args: &[String], current_index: usize) -> Result<String, TreeErrors> {
    if current_index + 1 < args.len() {
        let cleaned_ext = args[current_index + 1].trim_start_matches('.').to_lowercase();
        Ok(cleaned_ext)
    } else {
        Err(TreeErrors::MissingExtensionValue)
    }
}

/// Вывод справочной информации по работе утилиты
pub fn print_help() {
    println!("Oxi-Tree is a lightweight Rust-written utility for viewing directories and files as a tree\n");
    println!("Usage: oxi-tree <path> <flags>\n");
    println!("Flags:");
    println!("  -d,  --depth <number>     Limit directory traversal depth (default: 2)");
    println!("  -nd, --no-depth           Remove crawl depth limits (unlimited traversal)");
    println!("  -a,  --all                Include hidden files and folders (starting with .)");
    println!("  -h,  --help               Display help information and exit");
    println!("  -D,  --dirs-only          Display directories only (hide files)");
    println!("  -v,  --version            Show current application version");
    println!("  -e,  --ext <extension>    Filter files by extension (e.g., -e rs or -e .rs)");
    println!("  -U,  --unsorted           Disable natural sorting (traverse entries as-is for speed)");
    }

pub fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}
