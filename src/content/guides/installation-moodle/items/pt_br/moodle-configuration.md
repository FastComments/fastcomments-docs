A página de configurações do plugin está em **Administração do site > Plugins > Plugins locais > FastComments**. As opções disponíveis são:

#### Tenant ID

Seu ID de Tenant do FastComments. Encontre isso no <a href="https://fastcomments.com/auth/my-account" target="_blank">Painel do FastComments</a> em suas configurações de conta.

#### API Secret

Sua chave API Secret, necessária para o modo SSO Seguro. Encontre isso em <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Minha Conta > API Secret</a>.

#### Modo SSO

Escolha como os usuários são autenticados. Veja a seção [Modos de SSO](#items-moodle-sso-modes) para detalhes sobre cada opção.

- **Seguro** (recomendado) - autenticação assinada HMAC-SHA256 no lado do servidor
- **Simples** - dados do usuário no lado do cliente sem assinatura
- **Nenhum** - comentários anônimos, sem integração com login do Moodle

#### Contextos de Página

Controla onde os comentários aparecem:

- **Páginas do curso** - comentários nas páginas principais do curso
- **Páginas de módulo/atividade** - comentários em atividades e recursos individuais
- **Ambos** - comentários em todos os tipos de página

#### Estilo de Comentário

Escolha a experiência de comentários. Veja [Estilos de Comentário](#items-moodle-commenting-styles) para capturas de tela de cada modo.

- **Comentários** - widget de comentários encadeados padrão abaixo do conteúdo da página
- **Collab Chat** - discussões inline por seleção de texto com indicadores de presença
- **Ambos** - comentários e collab chat ativos juntos

#### URL do CDN

A URL do CDN do FastComments. Padrão para `https://cdn.fastcomments.com`. Altere isto para a URL do CDN da UE se seus dados estiverem hospedados na região da UE.