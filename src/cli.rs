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
}

/// Парсит аргументы командной строки для настройки параметров утилиты
/// Функция обрабатывает флаги и пути, переданные пользователем, и возвращает
/// структуру с настроенными параметрами, такими как путь и глубина обхода
pub fn parse_arg() -> Result<Args, TreeErrors> {
    let mut path = PathBuf::from(".");
    let mut depth = DEFAULT_MAX_DEPTH;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "-d" || args[i] == "--depth" {
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
        } else if !args[i].starts_with('-') {
            path = PathBuf::from(&args[i]);
        }
        i += 1;
    }

    Ok(Args { path, depth })
}
