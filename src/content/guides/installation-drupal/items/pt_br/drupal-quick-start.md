---
Esta é a versão 'muito longo; não li' das instruções do Drupal.

1. Instale o módulo com `composer require drupal/fcom`, ou coloque-o em `modules/custom/fastcomments/`.
2. Ative-o com `drush en fastcomments`, ou pela interface de administração em `/admin/modules`.
3. Vá para `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Insira seu Tenant ID e API Secret em [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Adicione o campo `FastComments` a qualquer tipo de conteúdo via `Structure > Content types > [type] > Manage fields`.

O módulo está publicado em [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---