O módulo adiciona três permissões do Drupal que você pode atribuir por função em `People > Permissions`.

- **Administer FastComments** - Acesso ao formulário de configurações do FastComments em `/admin/config/content/fastcomments`.
- **View FastComments** - Necessária para ver o widget de comentários. Sem essa permissão o widget não é renderizado.
- **Toggle FastComments** - Permite que usuários ativem ou desativem comentários por entidade usando o widget de campo.

Por padrão, somente usuários com a permissão `administer site configuration` podem alterar as configurações do FastComments. Conceda `View FastComments` a usuários anônimos e autenticados se você quiser que os visitantes vejam o widget.