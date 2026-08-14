use std::fs;
use std::path::Path;

pub fn display_tree(path: &Path) {
    let local_dir = fs::read_dir(path).unwrap();

    let mut dirs = Vec::new();
    let mut files = Vec::new();

    println!(".");

    for value in local_dir {
        let value = value.unwrap();
        let file_name = value.file_name();
        let file_type = value.file_type().unwrap();

        if file_type.is_dir() {
            dirs.push(file_name);
        } else {
            files.push(file_name);
        }
    }

    for dir in &dirs {
        print!("├──");
        println!("📁 {}/", dir.display());
    }

    for file in &files {
        print!("├──");
        println!("📄 {}", file.display());
    }

    println!("└── {} 📁  |  {} 📄", dirs.len(), files.len());
}
