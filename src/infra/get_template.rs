
use std::{collections::HashMap, fs, io};
use crate::{config, models::NorminTemplate};
use super::get_file_name::get_file_name;
pub fn get_template() -> io::Result<NorminTemplate> {
    let template_source = match fs::read_to_string(config::TEMPLATE_FILE_PATH) {
        Ok(value) => value,
        Err(e) => return Err(e)

    };
    let mut map = HashMap::new();
    let filename = get_file_name(config::TEMPLATE_FILE_PATH);
    map.insert(filename, template_source);
    Ok(NorminTemplate::from_template_map(map))
}