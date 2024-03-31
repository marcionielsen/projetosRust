
const SECOND_IN_MINUTES : u32 = 60;
const MINUTES_IN_HOUR : u32 = 60;
const SECONDS_IN_HOUR : u32 = SECOND_IN_MINUTES * MINUTES_IN_HOUR;

fn main() {
    // println!("Olá, mundo!");  -- primeiro programa em Rust

    // Variaveis e Constantes em Rust
    variaveis_constantes();

    // Tipos Primitivos 
    tipos_primiticos();
}

fn variaveis_constantes() {
    // Variaveis e Constantes em Rust

    let trabalhou : u32 = 30;
    println!("\nTrabalhou {} horas\n", trabalhou);

    let total_em_segundos : u32 = trabalhou * SECONDS_IN_HOUR;
    println!("Trabalhou {} segundos\n", total_em_segundos);  

}

fn tipos_primiticos() {
    // Tipos Primitivos 

    let a : i32 = -5;
    let b : u32 = 10;
    let c : f32 = 3.14;
    let d : char = 'a';

    println!("Tipos primitivos simples:\n - inteiros: {} \n - inteiros positivos: {} \n - pontos flutuantes: {}\n - caracter: {}\n", a, b, c, d);

    // compostos 
    let tupla = (1, 3.14, false, 'B');

    let vetor = [1,2,3,4,5];

    println!("Tipos primitivos compostos:\n- Tupla: {:?} \n- Matriz: {:?} \n", tupla, vetor);

}