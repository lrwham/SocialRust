use serde::{Deserialize, Serialize};
use crate::user::User;

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    content: String,
    author: User,
}

impl Post {
    fn new(content: String, author: User) -> Post {
        Post {
            content,
            author,
        }
    }

}