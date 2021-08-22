use diesel::pg::Pg;
use diesel::{debug_query, insert_into, RunQueryDsl};
use diesel_demo::establish_connection;
use diesel_demo::schema::posts::dsl::posts;

fn main() {
    let con = establish_connection();
    let q = insert_into(posts).default_values();
    println!("{}", debug_query::<Pg, _>(&q));
    let _ = q.execute(&con);
    println!("Default values inserted")
}
