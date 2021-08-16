use crate::{infra::get_flat_files, models::Post};
use std::fs;
use get_flat_files::get_flat_files;

pub fn get_posts() -> Vec<Post> {
    get_flat_files("./assets/post").iter()
        .map(|path| {
            let contents = fs::read_to_string(&path).unwrap();
            Post::from_md(path.to_str().unwrap(), markdown::to_html(&contents))
        })
        .collect::<Vec<Post>>()
}
