use diesel::{insert_into, ExpressionMethods, RunQueryDsl};
use diesel_demo::establish_connection;
use diesel_demo::schema::posts::columns::{body, publish_at, title, updated_at};
use diesel_demo::schema::posts::dsl::posts;
use std::time::SystemTime;

fn main() {
    let con = establish_connection();
    let r = insert_into(posts)
        .values((
            title.eq("hola"),
            body.eq("este es el body"),
            publish_at.eq(SystemTime::now()),
            updated_at.eq(SystemTime::now()),
        ))
        .execute(&con);
    println!("{:?}", r);
}
