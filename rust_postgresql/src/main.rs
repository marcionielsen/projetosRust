mod utils;
mod domain;
mod repository;

use postgres::SimpleQueryMessage::{Row, CommandComplete};

use crate::{repository::manager_db::ManagerDb, utils::{functions_dao::{excute_simple_query, imprimir_cabec, imprimir_result_set, separador}, queries::SQL}};

fn main() {

    println!("Conectando ao PostgreSQL...");

    let md : ManagerDb = ManagerDb::new(); // Encapsulando as configurações do banco no objeto ManagerDb

    let mut client = md.db_conn;

    println!(">>>Executando query...\n");

    let query_result = excute_simple_query(&mut client, SQL);

    let _query_result = match query_result {
        Err(execute_erro) => {
            eprintln!("\n### [ERROR] - {}\n", execute_erro);
        },
        Ok(query_results) => { 
            let mut nao_imprimiu_cabec = true;
            let mut tamanho_linha: u32 = 0;
            for resultado in query_results {
                match resultado {
                    Row(row) => {
                        
                        if nao_imprimiu_cabec {

                            tamanho_linha = imprimir_cabec(&row);
                            nao_imprimiu_cabec = false;
                        }
                        
                        imprimir_result_set(&row);
                    },
                    CommandComplete(num_rows) => { 
                        separador(tamanho_linha);

                        println!("{} registros\n", num_rows)
                    }, 
                    _ => {
                        println!("\n### [ERROR] - Erro inesperado!"); 
                    },
                }
            }
            println!(">>> Query executada com sucesso ...");
        },
    };

    let fechou: &Result<(), postgres::Error> = &client.close();

    match &fechou {
        Err(closed_erro) => {
            eprintln!("\n### [ERROR] - {}", closed_erro);
            return;
        },
        Ok(_) => println!("Conexão fechada ...")
    }
    
    println!("Finalizado...");
}
