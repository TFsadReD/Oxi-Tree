//! # Консольная утилита `tree` на языке Rust
//!
//! Точка входа в программу. Запускает обход и отображение
//! древовидной структуры директорий

mod tree;
mod errors;
mod cli;

use errors::TreeErrors;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), TreeErrors> {
    let args = cli::parse_arg()?;

    if args.show_help {
        cli::print_help();
        return Ok(());
    }

    if !args.path.exists() {
        return Err(errors::TreeErrors::NotFound(args.path.to_path_buf()));
    }

    if !args.path.is_dir() {
        return Err(errors::TreeErrors::NotADirectory(args.path.to_path_buf()));
    }

    tree::display_tree(&args.path, args.depth);

    Ok(())
}
