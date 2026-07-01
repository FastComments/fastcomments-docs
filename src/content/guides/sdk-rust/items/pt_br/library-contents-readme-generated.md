The FastComments Rust SDK consists of several modules:

- **Client Module** - Cliente API para as APIs REST do FastComments
  - Definições completas de tipos para todos os modelos da API
  - Três clientes API que cobrem todos os métodos do FastComments:
    - `default_api` (**DefaultApi**) - Métodos autenticados por chave de API para uso no lado do servidor
    - `public_api` (**PublicApi**) - Métodos públicos, sem chave de API, seguros para chamar a partir de navegadores e aplicativos móveis
    - `moderation_api` (**ModerationApi**) - Um conjunto extenso de APIs de moderação ao vivo e rápidas. Cada método de Moderação aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão da FastComments.com.
  - Suporte completo a async/await com tokio
  - Veja [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) para documentação detalhada da API

- **SSO Module** - Utilitários de Single Sign-On do lado do servidor
  - Geração segura de token para autenticação de usuário
  - Suporte para modos SSO simples e seguros
  - Assinatura de token baseada em HMAC‑SHA256

- **Core Types** - Definições de tipos compartilhados e utilitários
  - Modelos de comentário e estruturas de metadados
  - Configurações de usuário e locatário
  - Funções auxiliares para operações comuns