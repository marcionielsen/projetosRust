
const SECOND_IN_MINUTES : u32 = 60;
const MINUTES_IN_HOUR : u32 = 60;
const SECONDS_IN_HOUR : u32 = SECOND_IN_MINUTES * MINUTES_IN_HOUR;

fn main() {
    // println!("Olá, mundo!");  -- primeiro programa em Rust

    // Variaveis e Constantes em Rust

    let trabalhou : u32 = 30;
    println!("Trabalhou {} horas", trabalhou);

    let total_em_segundos : u32 = trabalhou * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos", total_em_segundos);

}
