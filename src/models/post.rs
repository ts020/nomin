#[derive(Debug)]
pub struct Post {
    pub text: String
}

impl Post {
    pub fn from_md(text: String) -> Post {
        Post{
            text,
        }
    }
}