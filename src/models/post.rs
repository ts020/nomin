use std::path::PathBuf;

use serde::ser::{Serialize, SerializeStruct, Serializer};
pub struct Post {
    pub path: PathBuf,
    pub text: String
}

impl Post {
    pub fn from_md(path:PathBuf, text: String) -> Post {
        Post{
            path,
            text,
        }
    }
}

impl Serialize for Post {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Post", 1)?;
        s.serialize_field("text", &self.text)?;
        s.end()
    }
}