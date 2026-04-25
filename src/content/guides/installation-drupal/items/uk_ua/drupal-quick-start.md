---
Це коротка версія інструкцій для Drupal (TL;DR).

1. Встановіть модуль командою `composer require drupal/fcom`, або помістіть його в `modules/custom/fastcomments/`.
2. Увімкніть його командою `drush en fastcomments`, або через інтерфейс адміністратора за адресою `/admin/modules`.
3. Перейдіть до `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Введіть свій Tenant ID та API Secret з [Налаштування > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЄС](https://eu.fastcomments.com/auth/my-account/api)).
5. Додайте поле `FastComments` до будь-якого типу вмісту через `Structure > Content types > [type] > Manage fields`.

Модуль опублікований на [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---