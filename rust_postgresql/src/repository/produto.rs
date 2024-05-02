use postgres::SimpleQueryMessage;

use crate::{domain::produto::Produto, utils::{functions_dao, queries}};
use super::manager_db::{self, ManagerDb};
use crate::repository;

impl repository::Repository  {
    
    pub fn all_produtos() -> Vec<Produto> {

        let mut produtos : Vec<Produto> = Vec::new();
        let mut mdb: ManagerDb = manager_db::ManagerDb::new();
        
        let resultado: Result<Vec<SimpleQueryMessage>, postgres::Error> = functions_dao::excute_simple_query(&mut mdb.db_conn, queries::SQL);

        let _dados = match resultado {

            Ok(lista_dados) => {

                for dados  in lista_dados {
                    
                    if let SimpleQueryMessage::Row(row) = dados  {
                        let produto = Produto::new(row.get(0).unwrap().to_string(), 
                        row.get(1).unwrap().to_string(), 
                        row.get(2).unwrap().to_string(), 
                        row.get(3).unwrap().to_string());           

                        produtos.push(produto);
                    }
                }
            },
            Err(erro_query) => {
                eprintln!("\n### [ERROR] - {}\n", erro_query);
            },
        };

        produtos
    }

}