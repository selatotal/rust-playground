#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use schema::posts::dsl::*;
use models::{Post,NewPost};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &SqliteConnection, t: &'a str, b: &'a str) -> Post {
    let new_post = NewPost { title: t, body: b };

    let inserted_count = diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving post");
    let response = posts
        .order(id.desc())
        .limit(inserted_count as i64)
        .load(conn)
        .expect("Error getting post")
        .into_iter()
        .rev()
        .collect::<Vec<Post>>();
    response[0].clone()
}

pub fn define_published<'a>(conn: &SqliteConnection, input: &'a Post) -> Post {
    diesel::update(posts.find(input.id))
        .set(published.eq(true))
        .execute(conn)
        .expect(&format!("Unable to find post with id {}", input.id));
    posts.find(input.id).first(conn).unwrap()
}
