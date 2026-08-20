//! # Обработка командной строки для утилиты `tree`
//!
//! Модуль для обработки аргументов командной строки, флагов,
//! путей и других параметров, передаваемых пользователем

use crate::errors::TreeErrors;

use std::{env, usize};
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
                if i + 1 < args.len() {
                    if let Ok(parsed_depth) = args[i + 1].parse::<usize>() {
                        depth = parsed_depth;
                        i += 2;
                        continue;
                    } else {
                        return Err(TreeErrors::InvalidDepth(args[i + 1].clone()));
                    }
                } else {
                    return Err(TreeErrors::InvalidDepth("значение отсутствует".to_string()));
                }
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
