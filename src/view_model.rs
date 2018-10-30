use serde_derive::Serialize;
use models::Post;

#[derive(Debug, Serialize)]
pub struct PostView {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub date: String,
    pub content: String,
    pub content_preview: String,
    pub published: bool,
}

pub fn post_view (post: &Post) -> PostView {
    let copy = post.content.clone();
    PostView {
        id: post.id,
        user_id: post.user_id,
        title: (post.title).clone(),
        date: post.date.format("%b %-d %Y").to_string(),
        content: (post.content).clone(),
        content_preview: copy[0..140].to_string(),
        published: post.published,
    }
}