# projetosRust
Projetos de aprendizagem da linguagem Rust

## Gerenciamento de memória em Rust
 | MEMORY             | CONTENTS                                            | SIZE              | LIFETIME             | CLEANUP                       |<br>
 |--------------------|-------|---------------------------------------------|-------------------|----------------------|-------------------------------|<br>
 | |----------------| |       |                                             |                   |                      |                               |<br>  
 | |STATIC          | |Static | - Binário do programa                       | Fixo              | Programa completo    | Quando o programa terminar    |<br>
 | |                | |       | - Variáveis Static                          |                   |                      |                               |<br>
 | |----------------| |       | - Literais String                           |                   |                      |                               |<br>
 | |----------------| |       | * saber o tamanho na compilação             |                   |                      |                               |<br>
 | | STACK          | |-------|---------------------------------------------|-------------------|----------------------|-------------------------------|<br>
 | |-------  -------| |Stack  | - Argumentos de funções                     | Dinâmico          | Funções              | Quando as funções terminarem  |<br>
 | |       \/       | |       | - Variáveis Locais                          | * limite superior |                      |                               |<br>
 | |----------------| |       | - Cada thread tem uma pilha                 |                   |                      |                               |<br>
 | |                | |       |   isolada                                   |                   |                      |                               |<br>
 | |                | |       | - Para cada função é criada um stack frame, |                   |                      |                               |<br>
 | |  FREE          | |       |   quando a função termina, o stack frame é  |                   |                      |                               |<br>
 | |                | |       |   desalocado, assim como todas as variáveis |                   |                      |                               |<br>
 | |       /\       | |       |   declaradas nele                           |                   |                      |                               |<br>
 | |-------  -------| |       | * saber o tamanho na compilação             |                   |                      |                               |<br>
 | | HEAP           | |-------|---------------------------------------------|-------------------|----------------------|-------------------------------|<br>
 | |                | |Heap   | - Valores vivem além das funções            | Dinâmico          | Definido pelo        | Manualmente                   |<br>
 | |                | |       | - Compartilhada através das threads         | * acima do limite | programador ou pela  | ou via GC                     |<br>
 | |                | |       | - Valores grandes                           |   do computador   | linguagem            | ou via RAII                   |<br>
 | |                | |       | - Valores de tamanhos dinâmicos             |                   |                      |                               |<br>
 | |                | |       | * Não sabe o tamanho na compilação          |                   |                      |                               |<br>
 | |                | |       |                                             |                   |                      |                               |<br>
 | |----------------| |       |                                             |                   |                      |                               |<br>  
