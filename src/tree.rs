use std::fs;
use std::path::Path;

pub fn display_tree(path: &Path) {
    let local_dir = fs::read_dir(path).unwrap();

    let mut dirs_count = 0;
    let mut files_count = 0;

    println!(".");

    for value in local_dir{
        let value = value.unwrap();
        let file_name = value.file_name();
        let file_type = value.file_type().unwrap();

        print!("├──");

        if file_type.is_dir(){
            dirs_count += 1;
            println!("📁 {}/", file_name.display());
        } else {
            files_count += 1;
            println!("📄 {}", file_name.display());
        }
    }

    println!("└── {} 📁  |  {} 📄", dirs_count, files_count);
}
