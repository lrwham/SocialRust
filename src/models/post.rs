use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    pub content: String,
    pub author: User,
}

impl Post {
    pub fn new(content: String, author: User) -> Post {
        Post {
            content,
            author,
        }
    }

}