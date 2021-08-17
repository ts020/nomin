// use std::fs::{File, create_dir};
// use std::io::Write;
use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use std::io::Write;

pub fn write_file(write_path: PathBuf, text: String) {
    let dir = write_path.parent().unwrap();
    create_dir_all(dir).unwrap();
    // let dd = dir.is_file();
    // if dir.to_path_buf().is_dir() {
    //     create_dir_all(dir).unwrap();
    // }
    

    let mut file = File::create(write_path).unwrap();
    write!(file, "{}", text).unwrap();
    file.flush().unwrap();
}

#[test]
fn テスト() {
    write_file(PathBuf::from("./temp/dd.text"), "test".to_string());
}