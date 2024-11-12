mod models;
mod database;

use std::io::Write;
use database::Database;
use models::user::User;
use models::post::Post;


fn main() {
    println!("Hello, world!");
    let user = User::new("Alice".to_string(), 1);
    let message = "Hello, world!".to_string();

    let post = Post::new(message, user.clone());

    let mut db = Database::new();
    db.add_user(user);
    db.add_post(post);

    db.write_to_file("db.json").unwrap();

    let post = db.get_post(2);

    match post {
        Ok(post) => {
            println!("Post: {}", post.content);
            println!("Author: {}", post.author.name);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
