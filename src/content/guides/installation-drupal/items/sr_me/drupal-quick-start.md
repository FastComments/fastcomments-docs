Ово је верзија Drupal упутстава „превише дуго, нисам читао“.

1. Инсталирајте модул са `composer require drupal/fcom`, или га поставите у `modules/custom/fastcomments/`.
2. Омогућите га са `drush en fastcomments`, или преко админ интерфејса на `/admin/modules`.
3. Идите на `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Унесите ваш Tenant ID и API Secret из [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕУ](https://eu.fastcomments.com/auth/my-account/api)).
5. Додајте поље `FastComments` у било који тип садржаја преко `Structure > Content types > [type] > Manage fields`.

Модул је објављен на [drupal.org/project/fcom](https://www.drupal.org/project/fcom).