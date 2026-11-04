A página de configurações do plugin está em **Administração do Site > Plugins > Plugins locais > FastComments**. As opções disponíveis são:

#### Tenant ID

Seu Tenant ID do FastComments. Encontre isso no <a href="https://fastcomments.com/auth/my-account" target="_blank">painel do FastComments</a> nas configurações da sua conta.

#### API Secret

Sua chave de API Secret, necessária para o modo Secure SSO. Encontre isso em <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Minha Conta > API Secret</a>.

#### SSO Mode

Escolha como os usuários são autenticados. Veja a seção [SSO Modes](#moodle-sso-modes) para detalhes sobre cada opção.

- **Secure** (recomendado) - autenticação assinada no servidor com HMAC-SHA256
- **Simple** - dados do usuário no lado do cliente sem assinatura
- **None** - comentários anônimos, sem integração com login do Moodle

#### Page Contexts

Controla onde os comentários aparecem:

- **Course pages** - comentários nas páginas principais dos cursos
- **Module/activity pages** - comentários em atividades e recursos individuais
- **Both** - comentários em todos os tipos de página

#### Commenting Style

Escolha a experiência de comentários. Veja [Commenting Styles](#moodle-commenting-styles) para capturas de tela de cada modo.

- **Comments** - widget padrão de comentários encadeados abaixo do conteúdo da página
- **Collab Chat** - discussões inline sobre seleção de texto com indicadores de presença
- **Both** - comentários e collab chat ativos juntos

#### CDN URL

A URL da CDN do FastComments. O padrão é `https://cdn.fastcomments.com`. Altere para a URL da CDN da UE se seus dados estiverem hospedados na região da UE.