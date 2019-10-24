#[macro_use]
extern crate diesel;
pub mod schema;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_connection()->Result<PgConnection,failure::Error>{
    dotenv::dotenv().ok();
    Ok(PgConnection::establish(&std::env::var("DATABASE_URL")?)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
