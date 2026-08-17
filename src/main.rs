//! # Консольная утилита `tree` на языке Rust
//!
//! Точка входа в программу. Запускает обход и отображение
//! древовидной структуры директорий

mod tree;
mod errors;

use errors::TreeErrors;

use std::env;
use std::path::Path;

fn main() -> Result<(), errors::TreeErrors> {
    let path_arg = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let path = Path::new(&path_arg);

    if !path.exists() {
        return Err(TreeErrors::NotFound(path.to_path_buf()));
    }

    if !path.is_dir() {
        return Err(TreeErrors::NotADirectory(path.to_path_buf()));
    }

    tree::display_tree(path);

    Ok(())
}
