---
### Componentes Obrigatórios

Para On-Prem, o FastComments consiste apenas em um servidor de aplicação e um banco de dados. Simplificamos a implantação de forma que
a aplicação possa atender todo o tráfego diretamente sem adicionar outros componentes.

O servidor de aplicação é fornecido em uma imagem Docker e pode ser implantado com qualquer solução de gerenciamento de contêineres.

O banco de dados, MongoDB, pode ser auto-hospedado ou fornecido por outro provedor como AWS DocumentDB ou MongoDB Atlas.

O FastComments atualmente é testado com MongoDB 7, no entanto nosso objetivo é ser compatível com DocumentDB para facilitar a implantação.

### Tamanhos de Instância

Você verá que o FastComments é bastante otimizado e não requer máquinas grandes para o próprio aplicativo para manter P99s baixos.

Todas as tarefas em lote e cron usam streaming para limitar o uso total de memória.

As tabelas abaixo para o servidor de aplicação e banco de dados podem ajudar no dimensionamento.

### Instâncias do Servidor de Aplicação


| Usuários Concorrentes | CPUs Totais do Cluster | Memória Total do Cluster |
|-----------------------|------------------------|--------------------------|
| 100                   | 1                      | 256mb                    |
| 1K                    | 2                      | 512mb                    |
| 10K                   | 8                      | 1gb                      |
| 100K                  | 32                     | 8gb                      |
| 1M                    | 64                     | 64gb                     |

Por exemplo, um único núcleo servindo cerca de 100 threads de comentário por segundo normalmente nunca usa mais de 250mb RSS.

### Instâncias do Servidor de Banco de Dados

Dimensionar o banco de dados depende do tamanho do conjunto de trabalho, que é a quantidade de dados que você acessa em um dado momento, bem como das requisições concorrentes.

O FastComments é relativamente gentil com o Mongo, no sentido de que, para as consultas mais quentes, ele usa index hints, streaming cursors, e possui limites de concorrência em várias áreas para evitar sobrecarga em sistemas a jusante.

O que segue é um guia geral sobre tamanhos de instância de banco de dados. **Observe que isso é __por instância__, não os recursos totais no cluster**.

| Usuários Concorrentes | Comentários Armazenados | CPUs Por Instância | Memória Por Instância |
|-----------------------|-------------------------|--------------------|-----------------------|
| 100                   | 1k                      | 1                  | 256mb                 |
| 1K                    | 5k                      | 2                  | 512mb                 |
| 10K                   | 100k                    | 8                  | 2gb                   |
| 100K                  | 500k                    | 16                 | 8gb                   |
| 1M                    | 5M                      | 32                 | 32gb                  |

As tabelas acima são estimativas conservadoras. Você pode achar que os requisitos reais diferem com base na sua configuração específica (tamanhos de página, volume de comentários, etc).

---