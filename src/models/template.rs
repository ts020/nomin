use std::{collections::HashMap, path::PathBuf};

pub struct NorminTemplate {
    pub page_template_map: HashMap<PathBuf, String>,
}
impl NorminTemplate {
    pub fn from_template_map(page_template_map: HashMap<PathBuf, String>) -> NorminTemplate {
        NorminTemplate{
            page_template_map,
        }
    }
}