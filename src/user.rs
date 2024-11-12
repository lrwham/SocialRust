use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    id: i32,
}

impl User {
    fn new(name: String, id: i32) -> User {
        User {
            name,
            id,
        }
    }
}