O FastComments Rust SDK consiste em vários módulos:

- **Client Module** - Cliente de API gerado automaticamente para as APIs REST do FastComments
  - Definições de tipos completas para todos os modelos de API
  - Endpoints tanto autenticados (`DefaultApi`) quanto públicos (`PublicApi`)
  - Suporte completo a async/await com tokio
  - Veja [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) para documentação detalhada da API

- **SSO Module** - Utilitários de Single Sign-On do lado do servidor
  - Geração segura de tokens para autenticação de usuários
  - Suporte tanto para modos SSO simples quanto seguros
  - Assinatura de tokens baseada em HMAC-SHA256

- **Core Types** - Definições de tipos e utilitários compartilhados
  - Modelos de comentário e estruturas de metadados
  - Configurações de usuário e tenant
  - Funções auxiliares para operações comuns