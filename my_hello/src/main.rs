
const SECOND_IN_MINUTES : u32 = 60;
const MINUTES_IN_HOUR : u32 = 60;
const SECONDS_IN_HOUR : u32 = SECOND_IN_MINUTES * MINUTES_IN_HOUR;

fn main() {
    // println!("Olá, mundo!");  -- primeiro programa em Rust

    // Variaveis e Constantes em Rust
    variaveis_constantes();

    // Tipos Primitivos 
    tipos_primiticos();

    // Gerenciamento de memória em Rust

    // +--------------------++-----------------------------------------------------+-------------------+----------------------+-------------------------------+
    // | MEMORY             || CONTENTS                                            | SIZE              | LIFETIME             | CLEANUP                       |
    // +--------------------++-----------------------------------------------------+-------------------+----------------------+-------------------------------+
    // | |----------------| ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|      
    // | |STATIC          | ||Static | - Binário do programa                       | Fixo              | Programa completo    | Quando o programa terminar    |
    // | |                | ||       | - Variáveis Static                          |                   |                      |                               |    
    // | |----------------| ||       | - Literais String                           |                   |                      |                               |    
    // | |----------------| ||       | * saber o tamanho na compilação             |                   |                      |                               |    
    // | | STACK          | ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|    
    // | |-------  -------| ||Stack  | - Argumentos de funções                     | Dinâmico          | Funções              | Quando as funções terminarem  |
    // | |       \/       | ||       | - Variáveis Locais                          | * limite superior |                      |                               |    
    // | |----------------| ||       | - Cada thread tem uma pilha                 |                   |                      |                               |    
    // | |                | ||       |   isolada                                   |                   |                      |                               |    
    // | |                | ||       | - Para cada função é criada um stack frame, |                   |                      |                               |    
    // | |  FREE          | ||       |   quando a função termina, o stack frame é  |                   |                      |                               |    
    // | |                | ||       |   desalocado, assim como todas as variáveis |                   |                      |                               |    
    // | |       /\       | ||       |   declaradas nele                           |                   |                      |                               |    
    // | |-------  -------| ||       | * saber o tamanho na compilação             |                   |                      |                               |    
    // | | HEAP           | ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|    
    // | |                | ||Heap   | - Valores vivem além das funções            | Dinâmico          | Definido pelo        | Manualmente                   |    
    // | |                | ||       | - Compartilhada através das threads         | * acima do limite | programador ou pela  | ou via GC                     |    
    // | |                | ||       | - Valores grandes                           |   do computador   | linguagem            | ou via RAII                   |    
    // | |                | ||       | - Valores de tamanhos dinâmicos             |                   |                      |                               |    
    // | |                | ||       | * Não sabe o tamanho na compilação          |                   |                      |                               |    
    // | |                | ||       |                                             |                   |                      |                               |   
    // | |----------------| ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|      
    // +--------------------++-----------------------------------------------------+-------------------+----------------------+-------------------------------+
    



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