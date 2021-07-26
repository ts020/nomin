mod config;
mod models;
mod infra;
fn main() {
    let posts = infra::get_posts();
    println!("{:?}", posts);
}
