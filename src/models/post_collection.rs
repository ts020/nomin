


use super::Post;

pub struct PostCollection {
    collecction: Vec<Post>
}

impl PostCollection {
    pub fn from(collection: Vec<Post>) -> PostCollection {
        PostCollection {
            collecction: collection
        }
    }

    pub fn find_by_path(&self, path: &str) -> Vec<&Post> {
        let replaceed_path = str::replace(path, "assets/template", "./assets/post")
            .replace(".hbs", "");

        return self.collecction.iter().filter(|post| -> bool {
            post.path.to_str().unwrap().to_string().starts_with(&replaceed_path)
        }).collect::<Vec<_>>()
    }
}