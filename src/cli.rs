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
    // pub show_hidden: bool,
    pub show_help: bool,
}

/// Парсит аргументы командной строки для настройки параметров утилиты
/// Функция обрабатывает флаги и пути, переданные пользователем, и возвращает
/// структуру с настроенными параметрами, такими как путь и глубина обхода
pub fn parse_arg() -> Result<Args, TreeErrors> {
    let mut path = PathBuf::from(".");
    let mut depth = DEFAULT_MAX_DEPTH;
    // let mut show_hidden = false;
    let mut show_help = false;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--depth" => {
                depth = parse_depth(&args, i)?;
                i += 2;
                continue;
            }

            "-nd" | "--no-depth" => {
                depth = usize::MAX;
            }

            // "-a" | "--all" => {
            //     show_hidden = true;
            // }

            "-h" | "--help" => {
                show_help = true;
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
        // show_hidden,
        show_help,
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

/// Вывод справочной информации по работе утилиты
pub fn print_help() {
    println!("Oxi-Tree is a lightweight Rust-written utility for viewing directories and files as a tree\n");
    println!("Usage: oxi-tree <path> <flags>\n");
    println!("Flags:");
    println!("  -d,  --depth <number>     Limit the crawl depth");
    println!("  -nd, --no-depth           Remove the crawl depth limit");
    println!("  -a,  --all                Show hidden files and folders");
    println!("  -h,  --help               Show this help");
}
