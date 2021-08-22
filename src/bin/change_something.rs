use diesel::RunQueryDsl;
use diesel::{QueryDsl, SaveChangesDsl};
use diesel_demo::establish_connection;
use diesel_demo::models::Post;
use diesel_demo::schema::posts::dsl::posts;

fn main() {
    let con = establish_connection();
    let mut target: Post = posts.find(1).get_result(&con).expect("failed to get 1st");
    println!("Just read {:?}", target);
    target.title = target.title + "changed";
    println!("just changed locally {:?}", target);
    target.save_changes::<Post>(&con);
    println!("saved the changes!!!");
    let target: Post = posts.find(1).first(&con).expect("failed to get 1st now");
    println!("does it update it? {:?}", target);
}
