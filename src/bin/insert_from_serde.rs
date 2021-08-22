use diesel::{insert_into, RunQueryDsl};
use diesel_demo::establish_connection;
use diesel_demo::models::{NewPost, Post};
use diesel_demo::schema::posts::dsl::posts;

fn main() {
    let json = r#"{ "title": "Sean", "body": "Black" }"#;
    let post_from_json = serde_json::from_str::<NewPost>(json).expect("not rigth");

    let con = establish_connection();
    let r = insert_into(posts).values(&post_from_json).execute(&con);
    println!("{:?}", r);
}
