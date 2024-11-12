use std::io::Write;
use crate::models::post::Post;
use crate::models::user::User;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Database{
    users: Vec<User>,
    posts: Vec<Post>,
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

    #[allow(dead_code)]
    pub fn get_user_by_id(&self, id: i32) -> Option<&User> {
        // find if user is in vec
        for user in &self.users {
            if user.get_id() == id {
                return Some(user);
            }
        }
        None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_to_file() {
        let mut db = Database::new();
        let user = User::new("Alice".to_string(), 1);
        let post = Post::new("Hello, world!".to_string(), user.clone(), 7);
        db.add_user(user);
        db.add_post(post.clone());
        db.write_to_file("test.json").unwrap();
        let file = std::fs::read_to_string("test.json").unwrap();
        let db2: Database = serde_json::from_str(&file).unwrap();

        // convert to JSON string
        let db = serde_json::to_string(&db).unwrap();
        let db2 = serde_json::to_string(&db2).unwrap();

        assert_eq!(db, db2);

        // remove file
        std::fs::remove_file("test.json").unwrap();
    }
}