
use postgres::{Client, NoTls, SimpleQueryMessage::{CommandComplete, Row}};


const DB_CONFIG: &str = "postgresql://postgres:123@localhost:5432/dbRust?application_name=RUST_POSTGRESQL_APP";


fn main() {

    println!("Conectando ao PostgreSQL...");

    let mut client = connect_db(DB_CONFIG);

    println!(">>>Executando query...\n");
    const SQL: &str = "SELECT id_produto, descricao, data_cadastro, preco_unitario FROM public.produto";

    let query_result = excute_simple_query(&mut client, SQL);

    let _query_result = match query_result {
        Err(execute_erro) => {
            eprintln!("\n### [ERROR] - {}\n", execute_erro);
            // return;
        },
        Ok(query_results) => { 
            let mut nao_imprimiu_cabec = true;
            let mut tamanho_linha = 0;
            for resultado in query_results {
                match resultado {
                    Row(row) => {
                        let colunas = row.len();

                        if nao_imprimiu_cabec {
                            (0..colunas).for_each(|colunas_idx| {                            
                                if let Some(valor) = row.columns().get(colunas_idx) {
                                    if colunas_idx == 0 {
                                        tamanho_linha += valor.name().len();
                                        tamanho_linha += 32 + 3;
                                        print!("| {} \t\t\t\t", valor.name());
                                    } else if colunas_idx == 2 {
                                        tamanho_linha += valor.name().len();
                                        tamanho_linha += 24 + 3;
                                        print!("| {} \t\t\t", valor.name());
                                    } else if colunas_idx == 3 {
                                        tamanho_linha += valor.name().len();
                                        tamanho_linha += 8 + 3;
                                        print!("| {} \t", valor.name());
                                    } else {
                                        tamanho_linha += valor.name().len();
                                        tamanho_linha += 24 + 3;
                                        print!("| {} \t\t\t", valor.name());
                                    }
                                }
                            });
                            print!("|\n");
                            tamanho_linha += 2;
                            tamanho_linha -= 11;

                            (0..tamanho_linha).for_each(|_contador|{
                                print!("-");
                            });
                            print!("\n");

                            nao_imprimiu_cabec = false;
                        }
                        
                        (0..colunas).for_each(|colunas_idx| {
                            
                            if let Some(valor) = row.get(colunas_idx) {
                                if colunas_idx == 0 {
                                    print!("| {}  ", valor);
                                } else if colunas_idx == 2 {
                                    print!("| {} \t", valor);
                                } else if colunas_idx == 3 {
                                    print!("| {} \t\t", valor);
                                } else {
                                    print!("| {} \t\t\t", valor);
                                }
                            }
                        });
                        print!("|\n");
                    },
                    CommandComplete(num_rows) => { 
                        (0..tamanho_linha).for_each(|_contador|{
                            print!("-");
                        });
                        print!("\n");

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


pub fn connect_db(p_db_config: &str) -> Client {
    let conn: Client = Client::connect(p_db_config, NoTls).expect("### [ERROR] - Falha ao conexão com o banco de dados!");

    conn 
}

pub fn excute_simple_query(client: &mut Client, p_sql: &str) -> Result<Vec<postgres::SimpleQueryMessage>, postgres::Error> {

    let query_result: Result<Vec<postgres::SimpleQueryMessage>, postgres::Error> = client.simple_query(p_sql);

    query_result
}
