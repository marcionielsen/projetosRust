use postgres::Client;

pub mod manager_db;
pub mod produto;

pub struct Repository {
    db: Client,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            db: manager_db::ManagerDb::new().db_conn,
        }
    }
}