extern crate diesel;
extern crate diesel_demo;

use diesel_demo::{create_post, establish_connection};
use std::io::stdin;

fn main() {
    let connection = establish_connection();
    println!("¿título?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("OK y ahora el cuerpo:");
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();
    let body = &body[..(body.len() - 1)];

    let post = create_post(&connection, title, body);
    println!("¡salvado todo!");
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
