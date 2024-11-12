use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub id: i32,
}

impl User {
    pub fn new(name: String, id: i32) -> User {
        User {
            name,
            id,
        }
    }
}