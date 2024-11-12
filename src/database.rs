use std::io::Write;
use crate::models::post::Post;
use crate::models::user::User;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Database{
    pub users: Vec<User>,
    pub posts: Vec<Post>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            users: Vec::new(),
            posts: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    /// Get a post by index safely checking if the index is in range
    /// return a result with the post or an error
    pub fn get_post(&mut self, i: usize) -> std::result::Result<Post, String> {
        if i < self.posts.len() {
            Ok(self.posts[i].clone())
        } else {
            Err("Index out of range".to_string())
        }
    }

    /// Write to file
    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = std::fs::File::create(filename)?;
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}