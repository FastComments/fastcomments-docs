Модулът FastComments за Drupal замества вградената в Drupal система за коментари с бърза система за коментиране в реално време. Модулът е [публикуван на drupal.org](https://www.drupal.org/project/fcom) и работи с Drupal 10 и 11.

Има два начина за инсталиране.

## Инсталиране с Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Ръчна инсталация

Изтеглете модула от [drupal.org/project/fcom](https://www.drupal.org/project/fcom) и го поставете в директорията `modules/custom/fastcomments/` на вашия сайт. След това го активирайте с `drush en fastcomments`, или чрез административния интерфейс на `Extend` (`/admin/modules`).

<sup>Бележка!</sup> Модулът зависи само от ядрото на Drupal (`user` и `field`). Не са необходими други Drupal модули или библиотеки.

След като модулът е активиран, отидете в секцията `Configuration`, за да зададете вашия Tenant ID и API Secret.