O SDK FastComments Swift consiste em vários módulos:

- **Client Module** - Cliente de API para as REST APIs do FastComments
  - Definições completas de tipos para todos os modelos de API
  - Métodos autenticados (`DefaultAPI`), públicos (`PublicAPI`) e de moderação (`ModerationAPI`)
  - Suporte completo a async/await
  - Veja [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) para documentação detalhada da API

- **SSO Module** - Utilitários de Single Sign-On do lado do servidor
  - Geração segura de tokens para autenticação de usuários
  - Suporte para modos SSO simples e seguro
  - Assinatura de token baseada em HMAC-SHA256 usando CryptoKit