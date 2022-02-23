use super::models::{Book, NewBook};
use rocket::{get,post,put,delete};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::value::Value;
use serde_json::json;
use crate::db::Pool;

#[get("/books" , format = "application/json")]
pub fn index(pool:&State<Pool>) -> Json<Value>{
    let conn = pool.get().unwrap();
    let books = Book::all(&conn);

    Json(json!({
        "status" : 200,
        "result" : books,
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
pub fn new(pool:&State<Pool>, new_book: Json<NewBook>) -> Json<Value> {
    let conn = pool.get().unwrap();
    Json(json!({
        "status": Book::insert(new_book.into_inner(), &conn),
        "result": null,
    }))
}

#[get("/books/<id>" , format = "application/json")]
pub fn show(pool:&State<Pool>, id: i32) -> Json<Value>{
    let conn = pool.get().unwrap();
    let result = Book::show(id,&conn);
    let status = if result.is_empty() {404} else { 200 };
    Json(json!({
        "status" : status,
        "result" : result.get(0),
    }))
}

#[put("/books/<id>", format = "application/json", data = "<book_new>")]
pub fn update(pool:&State<Pool>, id:i32, book_new: Json<NewBook>) -> Json<Value> {
    let conn = pool.get().unwrap();
    let status = if Book::update_by_id(id, &conn, book_new.into_inner()) {
        200
    }
    else { 404 };

    Json(json!({
        "status" : status,
        "result" : null,
    }))
}

#[delete("/books/<id>")]
pub fn delete(pool:&State<Pool>, id:i32) -> Json<Value>{
    let conn = pool.get().unwrap();
    let status = if Book::delete_by_id(id, &conn) {
        200
    }
    else { 404 };

    Json(json!({
        "status" : status,
        "result" : null,
    }))
}

#[get("/books/authors/<new_author>", format = "application/json")]
pub fn all_books_by_author(pool: &State<Pool>, new_author: String) -> Json<Value>{
    let conn = pool.get().unwrap();
    Json(json!({
        "status" : 200,
        "result" : Book::all_by_author(new_author, &conn),
    }))
}





