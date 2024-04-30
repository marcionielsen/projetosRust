use postgres::Client;

use crate::utils::{config, functions_dao};


pub struct ManagerDb {
    pub db_conn :Client,
}

impl ManagerDb {
    pub fn new() -> ManagerDb {
        ManagerDb {
            db_conn : functions_dao::connect_db(config::DB_CONFIG),
        }

    }
}