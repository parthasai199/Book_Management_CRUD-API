use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
//use std::ops::Deref;
//use rocket::State;
//use rocket::request::{self,Request,FromRequest};
//use rocket::outcome::Outcome;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url:String) -> Pool{
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("Database pool failure")
}




/*pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'r> FromRequest<'r> for Conn{
    type Error = ();
    fn from_request(request: &'r Request<'_>) -> request::Outcome<Conn, ()>{
        request.rocket().state::<Pool>().unwrap();
        match Pool.get(){
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn{
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}*/