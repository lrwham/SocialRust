use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    id: i32,
}

impl User {
    pub fn new(name: String, id: i32) -> User {
        User {
            name,
            id,
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}