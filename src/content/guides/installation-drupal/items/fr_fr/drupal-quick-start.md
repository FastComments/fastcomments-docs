---
Ceci est la version « trop longue ; pas lu » des instructions pour Drupal.

1. Installez le module avec `composer require drupal/fcom`, ou déposez-le dans `modules/custom/fastcomments/`.
2. Activez-le avec `drush en fastcomments`, ou depuis l'interface d'administration à `/admin/modules`.
3. Allez dans `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Saisissez votre Tenant ID et API Secret depuis [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Ajoutez le champ `FastComments` à n'importe quel type de contenu via `Structure > Content types > [type] > Manage fields`.

Le module est publié sur [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---