---
Это краткая версия инструкций для Drupal (TL;DR).

1. Установите модуль с помощью `composer require drupal/fcom`, или поместите его в `modules/custom/fastcomments/`.
2. Включите его с помощью `drush en fastcomments`, или через админ-панель по адресу `/admin/modules`.
3. Перейдите в `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Введите ваш Tenant ID и API Secret из [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Добавьте поле `FastComments` в любой тип содержимого через `Structure > Content types > [type] > Manage fields`.

Модуль опубликован на [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---