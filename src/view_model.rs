use models::Post;
use serde_derive::Serialize;
use comrak::{markdown_to_html, ComrakOptions};

#[derive(Debug, Serialize)]
pub struct PostView {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub date: String,
    pub pubdate: String,
    pub content: String,
    pub content_preview: String,
    pub published: bool,
}

pub fn post_view(post: &Post) -> PostView {
    let copy = post.content.clone();
    PostView {
        id: post.id,
        user_id: post.user_id,
        title: (post.title).clone(),
        date: post.date.format("%b %-d %Y").to_string(),
        pubdate: post.date.format("%a, %d %b %Y %T GMT").to_string(),
        content: markdown_to_html(&(post.content), &ComrakOptions::default()),
        content_preview: copy[0..250].to_string(),
        published: post.published,
    }
}



