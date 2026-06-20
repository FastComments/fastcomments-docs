O SDK Rust do FastComments consiste em vários módulos:

- **Client Module** - Cliente de API para as REST APIs do FastComments
  - Definições de tipo completas para todos os modelos de API
  - Três clientes de API cobrindo todos os métodos do FastComments:
    - `default_api` (**DefaultApi**) - métodos autenticados por chave de API para uso do lado do servidor
    - `public_api` (**PublicApi**) - métodos públicos, sem chave de API, seguros para serem chamados de navegadores e apps móveis
    - `moderation_api` (**ModerationApi**) - métodos que suportam o painel de moderador, incluindo moderação de comentários (listagem, contagem, pesquisa, logs, exportação), ações de moderação (remover/restaurar, sinalizar, definir status de revisão/spam/aprovação, votos, reabrir/fechar tópico), banimentos (banir a partir de um comentário, desfazer, resumos pré-ban, status/preferências de ban, contagens de usuários banidos), e insígnias & confiança (conceder/remover insígnias, insígnias manuais, obter/definir fator de confiança, perfil interno do usuário). Cada método de Moderação aceita um parâmetro `sso` para que a chamada possa ser feita em nome de um moderador autenticado por SSO.
  - Suporte completo a async/await com tokio
  - Consulte [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) para documentação detalhada da API

- **SSO Module** - Utilitários de Single Sign-On (SSO) do lado do servidor
  - Geração segura de tokens para autenticação de usuários
  - Suporte para modos SSO simples e seguros
  - Assinatura de tokens baseada em HMAC-SHA256

- **Core Types** - Definições de tipos compartilhados e utilitários
  - Modelos de comentário e estruturas de metadados
  - Configurações de usuário e tenant
  - Funções auxiliares para operações comuns