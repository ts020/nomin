use std::{fs::read_dir, path::PathBuf};

pub fn get_flat_files(path: &str) -> Vec<PathBuf> {
    read_dir(path).unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                return Some(get_flat_files(entry.path().to_str()?));
            } 
            Some(vec!(entry.path()))
        }).flatten()
        .collect::<Vec<_>>()
}

#[test]
fn ファイル取得テスト() {
    let files = get_flat_files("./assets/post");

    assert_eq!(files, vec![
        PathBuf::from("./assets/post/blog/post02.md"),
        PathBuf::from("./assets/post/blog/post01.md"),
        PathBuf::from("./assets/post/index.md")
    ]);
}