pub struct Template {
    pub text: String
}
impl Template {
    pub fn new(text: String) -> Post {
        Post{
            text: text,
        }
    }
}