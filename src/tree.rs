use std::fs;
use std::path::Path;

const DEFAULT_MAX_DEPTH: usize = 2;

pub fn display_tree(path: &Path) {
    println!("{}", path.display());

    let (total_dirs, total_files) = tree_recursive(path, "", 0, DEFAULT_MAX_DEPTH);

    println!("└── {} 📁  |  {} 📄", total_dirs, total_files);

}

pub fn tree_recursive(
    path: &Path,
    indent: &str,
    current_depth: usize,
    max_depth: usize,
) -> (usize, usize) {
    let local_dir = fs::read_dir(path).unwrap();

    let mut dirs = Vec::new();
    let mut files = Vec::new();

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
