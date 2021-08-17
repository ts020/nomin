use std::path::PathBuf;
use handlebars::Handlebars;
use serde_json::json;
use crate::{infra::write_file, models::{NorminTemplate, PostCollection}};

pub fn render_template(template: NorminTemplate, data: PostCollection) {
    let reg = Handlebars::new();
    for (file_path, template_source) in template.page_template_map.iter() {
        println!("ページ名 {}", file_path.display());
        let data_list = data.find_by_path(file_path.to_str().unwrap());
        for item in data_list {
            println!("itemPath: {}", item.path.display());
            let map = json!({
                "title": "頬ひお",
                "post": item.text
            });
            
            let html = reg.render_template(template_source, &map).unwrap();
            let write_path =str::replace(item.path.to_str().unwrap(), "assets/post", "./dist").replace(".md", "/index.html");
            write_file(PathBuf::from(write_path), html);
        }
    }

}