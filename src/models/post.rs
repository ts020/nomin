use serde::ser::{Serialize, SerializeStruct, Serializer};
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