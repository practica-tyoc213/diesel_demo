extern crate diesel;
extern crate diesel_demo;

use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}:{}", post.id, post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
