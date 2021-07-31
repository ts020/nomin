use handlebars::Handlebars;
use serde_json::json;
use crate::models::{NorminTemplate, Post};

pub fn render_template(template: NorminTemplate, data: &Vec<Post>) {
    let mut reg = Handlebars::new();

    for page_template in template.page_template_map.iter() {
        let option = reg.register_partial( page_template.0, page_template.1);
        println!("ページ名 {}", page_template.0);
        match option {
            Ok(_) => {
                let map = json!({
                    "title": "頬ひお",
                });
                reg.register_partial("post", &data[0].text).unwrap();
                let html = reg.render_template(page_template.1, &map).unwrap();
                println!("{}", html);
            },
            Err(e) => {
                println!("失敗 {}", e);
                return
            },
        };
    }

}