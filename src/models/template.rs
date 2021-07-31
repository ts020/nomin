use std::collections::HashMap;

pub struct NorminTemplate {
    pub page_template_map: HashMap<String, String>,
}
impl NorminTemplate {
    pub fn from_template_map(page_template_map: HashMap<String, String>) -> NorminTemplate {
        NorminTemplate{
            page_template_map,
        }
    }
}