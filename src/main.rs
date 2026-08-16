//! # Консольная утилита `tree` на языке Rust
//!
//! Точка входа в программу. Запускает обход и отображение
//! древовидной структуры директорий

mod tree;
use std::env;
use std::path::Path;

fn main() {
    let path_arg = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let path = Path::new(&path_arg);

    if !path.exists() {
        eprintln!("Ошибка: путь '{}' не существует!", path.display());
        std::process::exit(1);
    }

    if !path.is_dir() {
        eprintln!("Ошибка: '{}' не является директорией!", path.display());
        std::process::exit(2);
    }

    tree::display_tree(path);
}
