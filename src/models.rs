use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::book;
use crate::schema::book::dsl::book as all_books;

#[derive(Serialize, Queryable)]
pub struct Book
{
    pub book_id:i32,
    pub title: String,
    pub author: String,
}

#[derive( Deserialize, Insertable)]
#[table_name = "book"]
pub struct NewBook
{
    pub book_id:i32,
    pub title: String,
    pub author: String,
}

impl Book{
    pub fn show(book_id:i32, conn: &PgConnection) -> Vec<Book> {
        all_books
            .find(book_id)
            .load::<Book>(conn)
            .expect("Error loading the book")
    }

    pub fn all(conn: &PgConnection) -> Vec<Book> {
        all_books
            .order(book::book_id.asc())
            .load::<Book>(conn)
            .expect("Error loading all books")
    }

    pub fn update_by_id(_book_id: i32, conn: &PgConnection, books:NewBook) -> bool{
        use crate::schema::book::dsl::{author as a ,title as t};
        let NewBook {
            book_id,
            title,
            author,
        } = books;

        diesel::update(all_books.find(book_id))
            .set((a.eq(author),t.eq(title)))
            .get_result::<Book>(conn)
            .is_ok()
    }

    pub fn insert(books:NewBook, conn: &PgConnection) -> bool{
        diesel::insert_into(book::table)
            .values(&books)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(book_id: i32, conn: &PgConnection) -> bool{
        if Book::show(book_id, conn).is_empty(){
            return false;
        };
        diesel::delete(all_books.find(book_id))
            .execute(conn)
            .is_ok()
    }

    pub fn all_by_author(author: String, conn: &PgConnection) -> Vec<Book> {
        all_books
            .filter(book::author.eq(author))
            .load::<Book>(conn)
            .expect("Error find author books")
    }

}

