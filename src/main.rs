mod tree;
use std::path::Path;

fn main() {
    tree::display_tree(Path::new("src"));
}
