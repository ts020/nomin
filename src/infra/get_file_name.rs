use regex::Regex;

pub fn get_file_name(str:&str) -> String{
    let re = Regex::new(r"([^/]*)\.").unwrap();
    let m = re.captures(str).unwrap().get(1).unwrap();
    m.as_str().to_string()
}

#[test]
fn パス分割() {
    assert_eq!(get_file_name("aaa/bbb/ccc.txt"), "ccc".to_string());
}
