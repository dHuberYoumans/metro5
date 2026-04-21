use std::fs;
use std::path::PathBuf;

pub fn write_to_file(path: PathBuf, content: String) {
    let user_path = shellexpand::tilde(&path.to_string_lossy()).to_string();
    fs::File::create(&user_path).unwrap_or_else(|_| panic!("Could not open file {}", user_path));
    fs::write(&user_path, content)
        .unwrap_or_else(|_| panic!("Could not write to file {}", user_path));
}

#[test]
fn it_should_expand_tilde() {
    let expanded = shellexpand::tilde("~").to_string();
    assert!(!expanded.contains('~'));
    assert!(expanded.contains('/'));
}
