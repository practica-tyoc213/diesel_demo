use diesel::expression::dsl::now;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_demo::schema::posts::columns::publish_at;
use diesel_demo::schema::posts::dsl::posts;
use posts::dsl::*;

fn main() {
    let target = posts.filter(publish_at.lt(now));
    diesel::update(target).set(draft.eq(false));
}
