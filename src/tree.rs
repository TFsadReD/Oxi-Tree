//! Модуль для построения и форматированного вывода дерева файловой системы
//!
//! Предоставляет функционал для рекурсивного обхода папок, группировки
//! элементов по типам, форматированного вывода и подсчёта статистики

use std::fs;
use std::path::Path;

/// Максимальная глубина рекурсивного погружения по умолчанию
const DEFAULT_MAX_DEPTH: usize = 2;

/// Функция запускает рекурсивный поиск папок и файлов по указнному пути
pub fn display_tree(path: &Path) {
    println!("{}", path.display());

    let (total_dirs, total_files) = tree_recursive(path, "", 0, DEFAULT_MAX_DEPTH);

    println!("└── {} 📁  |  {} 📄", total_dirs, total_files);

}

/// Внутренняя рекурсивная функция для обхода содержимого директории
pub fn tree_recursive(
    path: &Path,
    indent: &str,
    current_depth: usize,
    max_depth: usize,
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

    for value in local_dir {
        let value = match value {
            Ok(e) => e,
            Err(_) => continue,
        };
        let file_type = match value.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        let file_name = value.file_name();

        if file_type.is_dir() {
            dirs.push(file_name);
        } else {
            files.push(file_name);
        }
    }

    let mut dir_count = dirs.len();
    let mut file_count = files.len();

    for dir in &dirs {
        println!("{}├── 📁 {}/", indent, dir.display());

        if current_depth + 1 < max_depth {
            let sub_path = path.join(dir);
            let new_indent = format!("{}│   ", indent);

            let (sub_dirs, sub_files) = tree_recursive(
                &sub_path,
                &new_indent,
                current_depth + 1,
                max_depth,
            );

            dir_count += sub_dirs;
            file_count += sub_files;
        }
    }

    for file in &files {
        println!("{}├── 📄 {}", indent, file.display());
    }

    (dir_count, file_count)
}
