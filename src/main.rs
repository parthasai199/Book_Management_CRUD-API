#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use std::env;
use dotenv::dotenv;
use rocket_codegen::routes;
use rocket::{Build, Rocket};
use crate::routes::*;


mod schema;
mod models;
mod db;
mod static_files;
mod routes;

fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Set DATABASE_URL");
    let pool = db::init_pool(database_url);
    rocket::build()
        .manage(pool)
        .mount(
            "/api/version-1",
            routes![index, new, show, update, delete, all_books_by_author],
        )
        .mount("/", routes![static_files::index]) //static_files::all

}

#[rocket::main]
async fn main()
{
    rocket().launch().await.ok();
}


