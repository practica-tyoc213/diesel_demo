use diesel::{QueryDsl, RunQueryDsl, TextExpressionMethods};
use diesel_demo::establish_connection;
use diesel_demo::schema::posts::columns::title;
use diesel_demo::schema::posts::dsl::posts;
use std::env::args;

fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting post");

    println!("se borraron {} posts", num_deleted);
}
