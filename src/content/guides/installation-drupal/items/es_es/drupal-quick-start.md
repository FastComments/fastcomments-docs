Esta es la versión 'TL;DR' de las instrucciones de Drupal.

1. Instala el módulo con `composer require drupal/fcom`, o colócalo en `modules/custom/fastcomments/`.
2. Actívalo con `drush en fastcomments`, o desde la interfaz de administración en `/admin/modules`.
3. Ve a `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Introduce tu Tenant ID y API Secret desde [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Añade el campo `FastComments` a cualquier tipo de contenido a través de `Structure > Content types > [type] > Manage fields`.

El módulo está publicado en [drupal.org/project/fcom](https://www.drupal.org/project/fcom).