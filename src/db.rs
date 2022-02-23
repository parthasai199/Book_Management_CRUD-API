use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url:String) -> Pool{
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("Database pool failure")
}
