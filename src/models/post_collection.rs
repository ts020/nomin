


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

        return self.collecction.iter().filter(|post| {
            let _path = post.path.to_str().unwrap().to_string();
            let d = _path.starts_with(&replaceed_path);
            let bo = d == true;
            bo
        }).collect::<Vec<_>>()
    }
}