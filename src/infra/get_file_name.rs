use std::path::PathBuf;

pub fn get_file_name(str:&str) -> String {
    let path = PathBuf::from(str);
    return path.file_stem().unwrap().to_os_string().to_str().unwrap().to_string();
}


#[test]
fn パス分割() {
    assert_eq!(get_file_name("aaa/bbb/ccc.txt"), "ccc");
}
