use diesel::pg::Pg;
use diesel_demo::schema::posts::dsl::*;

fn main() {
    let our_query = diesel::update(posts).set(draft.eq(false));
    println!("post are {}", debug_query::<Pg, _>(&our_query));
}
