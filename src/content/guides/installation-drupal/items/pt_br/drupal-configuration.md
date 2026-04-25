Todas as configurações ficam em `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obrigatório

- **Tenant ID** - Seu Tenant ID do FastComments. Encontre isto em [Configurações > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Necessário para SSO Seguro, verificação de webhooks e sincronização de páginas. Encontrado em [Configurações > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).

## Estilo de Comentários

Escolha o widget que corresponde à forma como você quer que as pessoas conversem em seu site.

- **Live Comments** - Comentários encadeados em tempo real.
- **Streaming Chat** - Interface de chat ao vivo, ideal para eventos e transmissões ao vivo.
- **Collab Chat** - Anotação por seleção de texto na área principal do conteúdo. Os visitantes destacam texto e iniciam uma discussão no contexto.
- **Collab Chat + Comments** - Tanto collab chat quanto comentários padrão na mesma página.

## Modo SSO

- **Nenhum** - Sem SSO. Usuários comentam como convidados ou criam uma conta FastComments.
- **Simples** - Transmite informações do usuário Drupal (name, email, avatar) para FastComments sem verificação do lado do servidor.
- **Seguro** - Usa HMAC-SHA256 para verificar usuários do Drupal com FastComments. Recomendado quando você tem um API Secret configurado.

Consulte a seção `Single Sign-On (SSO)` para detalhes.

## Outras Configurações

- **CDN URL** - Padrão: `https://cdn.fastcomments.com`.
- **Site URL** - Padrão: `https://fastcomments.com`.
- **Notificações por e-mail** - Envia um e-mail ao autor do conteúdo quando um novo comentário é postado em seu conteúdo.

Para residência de dados na UE, consulte a seção `EU Data Residency`.