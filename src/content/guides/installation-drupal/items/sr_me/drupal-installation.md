The FastComments Drupal модул замењује уграђене Drupal коментаре брзим, коментарским системом у реалном времену. Модул је [објављен на drupal.org](https://www.drupal.org/project/fcom) и ради са Drupal 10 и 11.

There are two ways to install it.

## Инсталирање помоћу Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Ручна инсталација

Преузмите модул са [drupal.org/project/fcom](https://www.drupal.org/project/fcom) и поставите га у ваш `modules/custom/fastcomments/` директоријум сајта. Затим га омогућите помоћу `drush en fastcomments`, или из администраторског интерфејса у `Extend` (`/admin/modules`).

<sup>Напомена!</sup> Модул зависи само од Drupal core-а (`user` и `field`). Не захтевају се додатни Drupal модули или библиотеке.

Once the module is enabled, head to the `Configuration` section to set up your Tenant ID and API Secret.

---