A página de configurações do plugin está em **Site Administration > Plugins > Local plugins > FastComments**. As opções disponíveis são:

#### Tenant ID

Seu FastComments Tenant ID. Encontre isto no <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments dashboard</a> nas configurações da sua conta.

#### API Secret

Sua chave API Secret, necessária para o modo Secure SSO. Encontre-a em <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>.

#### SSO Mode

Escolha como os usuários são autenticados. Veja a seção [SSO Modes](#moodle-sso-modes) para detalhes sobre cada opção.

- **Secure** (recommended) - autenticação assinada HMAC-SHA256 no lado do servidor
- **Simple** - dados do usuário no lado do cliente sem assinatura
- **None** - comentários anônimos, sem integração com login do Moodle

#### Page Contexts

Controla onde os comentários aparecem:

- **Course pages** - comentários nas páginas principais do curso
- **Module/activity pages** - comentários em atividades e recursos individuais
- **Both** - comentários em todos os tipos de página

#### Commenting Style

Escolha a experiência de comentários. Veja [Commenting Styles](#moodle-commenting-styles) para capturas de tela de cada modo.

- **Comments** - widget de comentários encadeados padrão abaixo do conteúdo da página
- **Collab Chat** - discussões em linha por seleção de texto com indicadores de presença
- **Both** - comentários e collab chat ativos simultaneamente

#### CDN URL

A URL do CDN do FastComments. O padrão é `https://cdn.fastcomments.com`. Altere isto para a URL do CDN da UE se seus dados estiverem hospedados na região da UE.