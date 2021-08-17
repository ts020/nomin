
use std::{collections::HashMap, fs, io};
use crate::models::NorminTemplate;
use super::get_flat_files::get_flat_files;
pub fn get_template() -> io::Result<NorminTemplate> {
    let mut map = HashMap::new();
    let template_files = get_flat_files("assets/template/");
    for path in template_files.into_iter() {
        let file = fs::read_to_string(path.as_os_str()).unwrap();
        map.insert(path, file);
    }
    Ok(NorminTemplate::from_template_map(map))
}