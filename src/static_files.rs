use std::io;
use rocket::fs::NamedFile;
use rocket::get;

#[get("/")]
pub async fn index() -> io::Result<NamedFile>{

    NamedFile::open("public/index.html").await
}


