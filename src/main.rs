mod models;
mod database;

use database::Database;
use models::user::User;
use models::post::Post;


fn main() {
    println!("Hello, world!");
    let user = User::new("Alice".to_string(), 1);
    let message = "Hello, world!".to_string();

    let post = Post::new(message, user.clone(), 7);

    let mut db = Database::new();
    db.add_user(user);
    db.add_post(post);

    db.write_to_file("db.json").unwrap();

    let post = db.get_post(2);

    match post {
        Ok(post) => {
            println!("Post: {}", post.get_content());
            println!("Author: {}", post.get_author().get_name());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
