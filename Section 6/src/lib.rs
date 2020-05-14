#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_connection() -> anyhow::Result<PgConnection> {
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
