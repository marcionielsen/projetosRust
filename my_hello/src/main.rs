use std::io;

const SECOND_IN_MINUTES : u32 = 60;
const MINUTES_IN_HOUR : u32 = 60;
const SECONDS_IN_HOUR : u32 = SECOND_IN_MINUTES * MINUTES_IN_HOUR;

fn main() {
    // println!("Olá, mundo!");  -- primeiro programa em Rust

    // Variaveis e Constantes em Rust
    variaveis_constantes();

    // Tipos Primitivos 
    tipos_primiticos();

    // Textos e caracteres
    textos_caracteres();

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

fn textos_caracteres() {

    // Exemplo de uso de String dinâmica
    let mut s = String::new();

    println!("=======[ Exemplo de uso de String dinâmica ]=======");
    println!("Digite um texto: ");
    io::stdin().read_line(&mut s).expect(" ***[ERROR]*** >  Error reading console!");

    println!("\nVocê digitou ==>> {}", s);
    
    // String Heap ou String Dinâmica ou somente String ou String owned
    let meu_nome = String::from("MÁRCIO");

    println!("String Heap ou String Dinâmica ou somente String ou String owned => {}\n", meu_nome);
    
    // str slice ou str reference
    let nome : &str = "Márcio";  

    println!("str slice ou str reference => {}\n", nome);

    // Concatenação de caracteres
    let l0 = 'M';
    let l1 = 'a';
    let l2 = 'r';
    let l3 = 'c';
    let l4 = 'i';
    let l5 = 'o';
    let l6 = '\n';

    println!("Concatenação de caracteres => {l0}{l1}{l2}{l3}{l4}{l5}{l6}"); 

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