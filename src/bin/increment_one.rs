extern crate diesel;
extern crate diesel_demo;

use self::models::Post;
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel_demo::schema::posts::columns::visit_count;
use diesel_demo::schema::posts::dsl::posts;
use diesel_demo::*;

fn main() {
    let x = diesel::update(posts).set(visit_count.eq(visit_count + 1));
    let con = establish_connection();
    x.execute(&con);
}
