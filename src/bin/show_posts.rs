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
        println!(
            "{} - {}:{}:[{:?}]",
            if post.published {
                "publicado"
            } else {
                "no publicado"
            },
            post.id,
            post.title,
            post.visit_count,
        );
        println!("-----------\n");
        println!("{}", post.body);
    }
}
