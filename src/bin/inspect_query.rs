use diesel::debug_query;
use diesel::pg::Pg;
use diesel::ExpressionMethods;
use diesel_demo::schema::posts::dsl::*;

fn main() {
    let our_query = diesel::update(posts).set(published.eq(false));
    println!("post are {}", debug_query::<Pg, _>(&our_query));
}
