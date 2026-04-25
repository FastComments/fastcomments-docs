FastComments integra-se ao sistema de usuários do Drupal através de SSO, ou single-sign-on. Seus usuários fazem login no seu site Drupal, e o módulo transmite automaticamente sua identidade ao FastComments. Sem contas extras para criar, sem sincronização inicial para executar.

O módulo oferece três modos de SSO, configurados em `Administration > Configuration > Content > FastComments`.

### Nenhum

Sem SSO. Os usuários comentam como convidados ou criam uma conta FastComments. Use isto se seu site for público e você não precisar vincular comentários a usuários do Drupal.

### Simples

Envia o nome, e-mail e avatar do usuário do Drupal para o FastComments sem verificação no servidor. Não é necessário um API Secret. Bom para sites internos ou de baixo risco.

### Seguro (recomendado)

Usa [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) para verificar cada identidade de usuário com o FastComments. Este é o modo recomendado quando você tem um API Secret configurado, e é o único modo que evita que um visitante se passe por outro usuário.

A identidade do usuário é enviada ao FastComments cada vez que um usuário visualiza um tópico de comentários. Não há sincronização inicial ou contínua que precise ser executada.

<sup>(Optional)</sup> Adicione seus administradores a [Users & Administrators](https://fastcomments.com/auth/my-account/users) e moderadores a [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators) para melhorar a experiência deles e habilitar o rastreamento de estatísticas para moderadores.

Para uma visão mais detalhada de como o SSO funciona, veja a [seção SSO](/guide-customizations-and-configuration.html#sso) da documentação de personalizações.