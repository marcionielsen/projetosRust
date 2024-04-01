# projetosRust
Projetos de aprendizagem da linguagem Rust

---
## Gerenciamento de memória em Rust


| CONTENTS                                             | SIZE              | LIFETIME             | CLEANUP                       |
|---|---|---|---|
|        |                                             |                   |                      |                               |
| Static | - Binário do programa                       | Fixo              | Programa completo    | Quando o programa terminar    |
|        | - Variáveis Static                          |                   |                      |                               |
|        | - Literais String                           |                   |                      |                               |
|        | * saber o tamanho na compilação             |                   |                      |                               |
|--------|---------------------------------------------|-------------------|----------------------|-------------------------------|
| Stack  | - Argumentos de funções                     | Dinâmico          | Funções              | Quando as funções terminarem  |
|        | - Variáveis Locais                          | * limite superior |                      |                               |
|        | - Cada thread tem uma pilha                 |                   |                      |                               |
|        |   isolada                                   |                   |                      |                               |
|        | - Para cada função é criada um stack frame, |                   |                      |                               |
|        |   quando a função termina, o stack frame é  |                   |                      |                               |
|        |   desalocado, assim como todas as variáveis |                   |                      |                               |
|        |   declaradas nele                           |                   |                      |                               |
|        | * saber o tamanho na compilação             |                   |                      |                               |
|--------|---------------------------------------------|-------------------|----------------------|-------------------------------|
| Heap   | - Valores vivem além das funções            | Dinâmico          | Definido pelo        | Manualmente                   |
|        | - Compartilhada através das threads         | * acima do limite | programador ou pela  | ou via GC                     |
|        | - Valores grandes                           |   do computador   | linguagem            | ou via RAII                   |
|        | - Valores de tamanhos dinâmicos             |                   |                      |                               |
|        | * Não sabe o tamanho na compilação          |                   |                      |                               |
|        |                                             |                   |                      |                               |
|        |                                             |                   |                      |                               |
