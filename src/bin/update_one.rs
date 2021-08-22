use diesel::expression::dsl::now;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_demo::schema::posts::columns::{publish_at, published, updated_at};
use diesel_demo::schema::posts::dsl::posts;
use std::time::SystemTime;

fn main() {
    let target = posts.filter(publish_at.lt(now));
    diesel::update(target).set((published.eq(false), updated_at.eq(SystemTime::now())));
}
