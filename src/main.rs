use std::fs;

fn main() {
    let local_dir = fs::read_dir(".").unwrap();

    println!(".");

    for value in local_dir{
        let value = value.unwrap();
        let file_name = value.file_name();
        let file_type = value.file_type().unwrap();

        print!("├──");

        if file_type.is_dir(){
            println!("📁 {}/", file_name.display());
        } else {
            println!("📄 {}", file_name.display());
        }
    }

    println!("└── Конец разметки.");
}
