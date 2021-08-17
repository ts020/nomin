use crate::{infra::get_flat_files, models::{Post, PostCollection}};
use std::fs;
use get_flat_files::get_flat_files;

pub fn get_posts() -> PostCollection {
    let list = get_flat_files("./assets/post").iter()
    .map(|path| {
        let contents = fs::read_to_string(&path).unwrap();
        Post::from_md(path.to_path_buf(), markdown::to_html(&contents))
    })
    .collect::<Vec<Post>>();
    PostCollection::from(list)
    
}
