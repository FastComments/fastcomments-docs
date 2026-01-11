---
O SDK FastComments para Swift consiste em vários módulos:

- **Client Module** - Cliente de API gerado automaticamente para as REST APIs do FastComments
  - Definições completas de tipos para todos os modelos de API
  - Endpoints tanto autenticados (`DefaultAPI`) quanto públicos (`PublicAPI`)
  - Suporte completo a async/await
  - Veja [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) para documentação detalhada da API

- **SSO Module** - Utilitários de Single Sign-On do lado do servidor
  - Geração segura de tokens para autenticação de usuários
  - Suporte para modos SSO tanto simples quanto seguros
  - Assinatura de tokens baseada em HMAC-SHA256 usando CryptoKit
---