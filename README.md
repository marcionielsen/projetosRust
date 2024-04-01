# projetosRust
Projetos de aprendizagem da linguagem Rust

## Gerenciamento de memória em Rust
 +--------------------++-----------------------------------------------------+-------------------+----------------------+-------------------------------+
 | MEMORY             || CONTENTS                                            | SIZE              | LIFETIME             | CLEANUP                       |
 +--------------------++-----------------------------------------------------+-------------------+----------------------+-------------------------------+
 | |----------------| ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|      
 | |STATIC          | ||Static | - Binário do programa                       | Fixo              | Programa completo    | Quando o programa terminar    |
 | |                | ||       | - Variáveis Static                          |                   |                      |                               |    
 | |----------------| ||       | - Literais String                           |                   |                      |                               |    
 | |----------------| ||       | * saber o tamanho na compilação             |                   |                      |                               |    
 | | STACK          | ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|    
 | |-------  -------| ||Stack  | - Argumentos de funções                     | Dinâmico          | Funções              | Quando as funções terminarem  |
 | |       \/       | ||       | - Variáveis Locais                          | * limite superior |                      |                               |    
 | |----------------| ||       | - Cada thread tem uma pilha                 |                   |                      |                               |    
 | |                | ||       |   isolada                                   |                   |                      |                               |    
 | |                | ||       | - Para cada função é criada um stack frame, |                   |                      |                               |    
 | |  FREE          | ||       |   quando a função termina, o stack frame é  |                   |                      |                               |    
 | |                | ||       |   desalocado, assim como todas as variáveis |                   |                      |                               |    
 | |       /\       | ||       |   declaradas nele                           |                   |                      |                               |    
 | |-------  -------| ||       | * saber o tamanho na compilação             |                   |                      |                               |    
 | | HEAP           | ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|    
 | |                | ||Heap   | - Valores vivem além das funções            | Dinâmico          | Definido pelo        | Manualmente                   |    
 | |                | ||       | - Compartilhada através das threads         | * acima do limite | programador ou pela  | ou via GC                     |    
 | |                | ||       | - Valores grandes                           |   do computador   | linguagem            | ou via RAII                   |    
 | |                | ||       | - Valores de tamanhos dinâmicos             |                   |                      |                               |    
 | |                | ||       | * Não sabe o tamanho na compilação          |                   |                      |                               |    
 | |                | ||       |                                             |                   |                      |                               |   
 | |----------------| ||-------|---------------------------------------------|-------------------|----------------------|-------------------------------|      
 +--------------------++-----------------------------------------------------+-------------------+----------------------+-------------------------------+
