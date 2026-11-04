Todas as configurações ficam em `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Obrigatórias

- **Tenant ID** - Seu Tenant ID do FastComments. Encontre em [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Necessário para SSO Seguro, verificação de webhooks e sincronização de páginas. Encontrado em [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).

## Armazenando o API Secret

O módulo lê o API Secret da primeira destas fontes que tiver um valor:

1. **Módulo Key** - Instale o [Key](https://www.drupal.org/project/key), crie uma chave do tipo *Authentication* e selecione-a em **API Secret key** no formulário de configurações. A chave pode ser respaldada por uma variável de ambiente ou por um arquivo fora da raiz web.
2. **settings.php** - Adicione `$config['fastcomments.settings']['api_secret'] = 'your-secret';` ao `settings.php` ou `settings.local.php`. O segredo nunca entra na configuração exportada.
3. **O formulário de configurações** - Insira-o no campo **API Secret**. Isso armazena o valor em `fastcomments.settings`, então ele acaba nas exportações de configuração. O relatório de status do Drupal emite um aviso quando esta é a fonte ativa.

O formulário mostra qual fonte está ativa. Deixar o campo em branco mantém o segredo armazenado. Marque **Remove the API Secret stored in configuration** para excluí-lo após mover o segredo para o módulo Key ou para o `settings.php`.

## Estilo de Comentários

Escolha o widget que corresponde à forma como você quer que as pessoas conversem no seu site.

- **Live Comments** - Comentários encadeados em tempo real.
- **Streaming Chat** - Interface de chat ao vivo, boa para eventos e transmissões ao vivo.
- **Collab Chat** - Anotação por seleção de texto na área principal de conteúdo. Visitantes destacam texto e iniciam uma discussão no contexto.
- **Collab Chat + Comments** - Tanto collab chat quanto comentários padrão na mesma página.

## Modo SSO

- **None** - Sem SSO. Os usuários comentam como convidados ou criam uma conta FastComments.
- **Simple** - Passa as informações do usuário do Drupal (nome, e-mail, avatar) para o FastComments sem verificação no lado do servidor.
- **Secure** - Usa HMAC-SHA256 para verificar usuários do Drupal com o FastComments. Recomendado quando você tem um API Secret configurado.

Consulte a seção `Single Sign-On (SSO)` para mais detalhes.

## Outras Configurações

- **CDN URL** - O padrão é `https://cdn.fastcomments.com`.
- **Site URL** - O padrão é `https://fastcomments.com`.
- **Notificações por e-mail** - Envia um e-mail ao autor do conteúdo quando um novo comentário é publicado em seu conteúdo.

Para residência de dados na UE, consulte a seção `EU Data Residency`.