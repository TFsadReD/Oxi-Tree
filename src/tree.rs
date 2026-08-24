//! Модуль для построения и форматированного вывода дерева файловой системы
//!
//! Предоставляет функционал для рекурсивного обхода папок, группировки
//! элементов по типам, форматированного вывода и подсчёта статистики

use crate::cli::Args;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Функция запускает рекурсивный поиск папок и файлов по указнному пути
pub fn display_tree(args: &Args) {
    println!("{}", args.path.display());

    let (total_dirs, total_files) = tree_recursive(&args.path, args, 0, "");

    if args.dirs_only {
        println!("└── {} 📁", total_dirs);
    } else {
        println!("└── {} 📁  |  {} 📄", total_dirs, total_files);
    }
}

/// Внутренняя рекурсивная функция для обхода содержимого директории
pub fn tree_recursive(
    path: &Path,
    args: &Args,
    current_depth: usize,
    indent: &str,
) -> (usize, usize) {
    let local_dir = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => {
            eprintln!("{}├── 🚫 [Отказано в доступе]", indent);
            return (0, 0);
        }
    };

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    for entry in local_dir.flatten() {
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        let file_name = entry.file_name();

        if file_type.is_dir() {
            if should_include_dir(&file_name, args) {
                dirs.push(file_name);
            }
        } else if !args.dirs_only {
            if should_include_file(&file_name, args) {
                files.push(file_name);
            }
        }
    }

    let mut dir_count = dirs.len();
    let mut file_count = files.len();

    for dir in &dirs {
        println!("{}├── 📁 {}/", indent, dir.to_string_lossy());

        if current_depth + 1 < args.depth {
            let sub_path = path.join(dir);
            let new_indent = format!("{}│   ", indent);

            let (sub_dirs, sub_files) = tree_recursive(
                &sub_path,
                args,
                current_depth + 1,
                &new_indent,
            );

            dir_count += sub_dirs;
            file_count += sub_files;
        }
    }

    for file in &files {
        println!("{}├── 📄 {}", indent, file.to_string_lossy());
    }

    (dir_count, file_count)
}

/// Вспомогательная функция: проверяет, нужно ли отображать папку
fn should_include_dir(dir_name: &OsStr, args: &Args) -> bool {
    let name_str = dir_name.to_string_lossy();
    if !args.show_hidden && name_str.starts_with('.') {
        return false;
    }
    true
}

/// Вспомогательная функция: проверяет, удовлетворяет ли файл всем флагам фильтрации
fn should_include_file(file_name: &OsStr, args: &Args) -> bool {
    let name_str = file_name.to_string_lossy();

    if !args.show_hidden && name_str.starts_with('.') {
        return false;
    }

    if let Some(ref target_ext) = args.ext {
        let path = Path::new(file_name);

        match path.extension() {
            Some(ext) => {
                if ext.to_string_lossy().to_lowercase() != *target_ext {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}
