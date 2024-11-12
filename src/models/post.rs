use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    content: String,
    author: User,
    id: i32,
}

impl Post {
    pub fn new(content: String, author: User, id: i32) -> Post {
        Post {
            content,
            author,
            id,
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[allow(dead_code)]
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    #[allow(dead_code)]
    pub fn get_author(&self) -> User {
        self.author.clone()
    }

}