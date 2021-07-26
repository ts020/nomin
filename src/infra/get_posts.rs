use crate::models::Post;
use std::fs;
use crate::config;
pub fn get_posts() -> Vec<Post> {
    let contents = fs::read_to_string(config::FILE_NAME)
        .expect("Something went wrong reading the file");
    vec![Post::from_md(markdown::to_html(&contents))]
}