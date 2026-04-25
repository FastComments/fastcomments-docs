---
Това е "твърде дълго; не прочетох" версия на инструкциите за Drupal.

1. Инсталирайте модула с `composer require drupal/fcom`, или го поставете в `modules/custom/fastcomments/`.
2. Активирайте го с `drush en fastcomments`, или от административния интерфейс на `/admin/modules`.
3. Отидете на `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Въведете вашия Tenant ID и API Secret от [Настройки > API/SSO](https://fastcomments.com/auth/my-account/api) ([ЕС](https://eu.fastcomments.com/auth/my-account/api)).
5. Добавете полето `FastComments` към всеки тип съдържание чрез `Structure > Content types > [type] > Manage fields`.

Модулът е публикуван на [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---