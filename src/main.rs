mod models;
mod database;

use database::Database;
use models::user::User;
use models::post::Post;


fn main() {
    println!("Welcome to Rust Social");
    println!("To get started, create the first user!");
    println!("Enter your name:");

    let mut name = String::new();

    std::io::stdin().read_line(&mut name).unwrap();

    let name = name.trim().to_string();

    let user = User::new(name, 1);


    let mut db = Database::new();
    db.add_user(user);

    println!("User created! Now the main menu.");

    loop {
        println!("1. Create a post");
        println!("2. View all posts");
        println!("3. Exit");

        let mut choice = String::new();

        std::io::stdin().read_line(&mut choice).unwrap();

        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter your post:");
                let mut content = String::new();
                std::io::stdin().read_line(&mut content).unwrap();
                let content = content.trim().to_string();
                let user = db.get_user_by_id(1).unwrap().clone();
                let post = Post::new(content, user, 1);
                db.add_post(post);
                println!("Post created!");
            }
            "2" => {
                for post in db.get_posts().iter() {
                    println!("{}: {}", post.get_author().get_name(), post.get_content());
                }
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
