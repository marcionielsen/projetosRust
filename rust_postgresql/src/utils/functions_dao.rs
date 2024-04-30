use postgres::{Client, NoTls, SimpleQueryRow};


pub fn connect_db(p_db_config: &str) -> Client {
    let conn: Client = Client::connect(p_db_config, NoTls).expect("### [ERROR] - Falha ao conexão com o banco de dados!");

    conn 
}

pub fn excute_simple_query(client: &mut Client, p_sql: &str) -> Result<Vec<postgres::SimpleQueryMessage>, postgres::Error> {

    let query_result: Result<Vec<postgres::SimpleQueryMessage>, postgres::Error> = client.simple_query(p_sql);

    query_result
}

pub fn imprimir_cabec(row : &SimpleQueryRow) -> u32 {
    let mut tamanho_linha: u32 = 0;

    let cols = row.columns();
    let count_cols = cols.len();

    (0..count_cols).for_each(|colunas_idx| {                            
        if let Some(valor) = row.columns().get(colunas_idx) {

            if colunas_idx == 0 {
                tamanho_linha += valor.name().len() as u32;
                tamanho_linha += 32 + 3;
                print!("| {} \t\t\t\t", valor.name());
            } else if colunas_idx == 2 {
                tamanho_linha += valor.name().len() as u32;
                tamanho_linha += 24 + 3;
                print!("| {} \t\t\t", valor.name());
            } else if colunas_idx == 3 {
                tamanho_linha += valor.name().len() as u32;
                tamanho_linha += 8 + 3;
                print!("| {} \t", valor.name());
            } else {
                tamanho_linha += valor.name().len() as u32;
                tamanho_linha += 24 + 3;
                print!("| {} \t\t\t", valor.name());
            }
        }
    });
    print!("|\n");
    tamanho_linha += 2;
    tamanho_linha -= 11;

    separador(tamanho_linha); 

    tamanho_linha

}

pub fn imprimir_result_set(row : &SimpleQueryRow) {

    let count_cols = row.len();
        
    (0..count_cols).for_each(|colunas_idx| {
        
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
}

pub fn separador(tamanho_linha: u32) {

    (0..tamanho_linha).for_each(|_contador: u32|{
        print!("-");
    });
    print!("\n");
}
