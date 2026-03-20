Navegue até **Administração > Configuração > Conteúdo > FastComments** (`/admin/config/content/fastcomments`).

### Configurações

- **Tenant ID** (obrigatório) - Seu Tenant ID do FastComments. Encontre isso em [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Necessário para SSO Seguro, verificação de webhooks e sincronização de páginas. Encontrado em [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **Modo SSO** - Integração de Single Sign-On:
  - **None** - Sem SSO, os usuários comentam como convidados ou criam contas FastComments.
  - **Simple** - Envia informações do usuário do Drupal (nome, email, avatar) para o FastComments sem verificação no servidor.
  - **Secure** - Usa verificação HMAC-SHA256 para autenticar com segurança usuários do Drupal no FastComments (recomendado).
- **Estilo de Comentário** - O tipo de widget a ser exibido:
  - **Live Comments** - Comentários encadeados em tempo real.
  - **Streaming Chat** - Interface de chat ao vivo.
  - **Collab Chat** - Anotação colaborativa por seleção de texto na área principal de conteúdo.
  - **Collab Chat + Comments** - Tanto collab chat quanto comentários padrão.
- **CDN URL** - URL do CDN do FastComments (padrão: `https://cdn.fastcomments.com`).
- **Site URL** - URL do site do FastComments (padrão: `https://fastcomments.com`).
- **Notificações por email** - Enviar um email aos autores do conteúdo quando um novo comentário for postado em seu conteúdo.

### Adicionando Comentários a Tipos de Conteúdo

Adicione o campo **FastComments** aos seus tipos de conteúdo via **Estrutura > Tipos de conteúdo > [type] > Gerenciar campos**. O campo tem um controle de status e um identificador personalizado opcional por entidade.

### Residência de Dados na UE

Para residência de dados na UE, atualize:
- **CDN URL** para `https://cdn-eu.fastcomments.com`
- **Site URL** para `https://eu.fastcomments.com`