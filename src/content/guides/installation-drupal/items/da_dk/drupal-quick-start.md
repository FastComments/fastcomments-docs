---
Dette er "for langt; gad ikke læse"-versionen af Drupal-instruktionerne.

1. Installer modulet med `composer require drupal/fcom`, eller læg det i `modules/custom/fastcomments/`.
2. Aktivér det med `drush en fastcomments`, eller fra admin-UI'en på `/admin/modules`.
3. Gå til `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Indtast dit Tenant ID og API Secret fra [Indstillinger > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Tilføj feltet `FastComments` til enhver indholdstype via `Structure > Content types > [type] > Manage fields`.

Modulet er udgivet på [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---