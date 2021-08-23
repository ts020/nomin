mod models;
mod infra;
mod view;
fn main()  {
    let posts = infra::get_posts();
    let template = infra::get_template();
    if let Ok(temp) = template { view::render_template(temp, posts) }
}
