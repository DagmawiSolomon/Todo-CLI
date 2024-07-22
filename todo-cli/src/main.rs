extern crate diesel;
extern crate dotenvy;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod schema;
pub mod models;

fn main() {
    dotenv().ok(); // Load the .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    // Example: Querying the database
    use self::schema::users::dsl::*;

    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
        println!("----------\n");
        println!("{}", user.email);
    }
}
