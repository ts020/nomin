mod models;
mod infra;
mod view;
fn main()  {
    let posts = infra::get_posts();
    let template = infra::get_template();
    match template {
        Ok(temp) => view::render_template(temp, posts),
        Err(_) => return
    }

}
