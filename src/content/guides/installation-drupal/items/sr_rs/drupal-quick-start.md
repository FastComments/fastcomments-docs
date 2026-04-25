---
Ово је „превише дуго; нисам читао“ (TL;DR) верзија Drupal упутстава.

1. Инсталирајте модул помоћу `composer require drupal/fcom`, или га ставите у `modules/custom/fastcomments/`.
2. Омогућите га помоћу `drush en fastcomments`, или из админ интерфејса на `/admin/modules`.
3. Идите на `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Унесите ваш Tenant ID и API Secret са [Подешавања > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Додајте поље `FastComments` у било који тип садржаја преко `Structure > Content types > [type] > Manage fields`.

Модул је објављен на [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---