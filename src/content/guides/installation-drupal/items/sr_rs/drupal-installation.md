The FastComments модул за Drupal замењује уграђене коментаре Drupala брзим системом за коментаре у реалном времену. Модул је [објављен на drupal.org](https://www.drupal.org/project/fcom) и ради са Drupal 10 и 11.

Постоје два начина за инсталирање.

## Инсталирање помоћу Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Ручна инсталација

Преузмите модул са [drupal.org/project/fcom](https://www.drupal.org/project/fcom) и поставите га у директоријум вашег сајта `modules/custom/fastcomments/`. Затим га омогућите помоћу `drush en fastcomments`, или из администраторског интерфејса у одељку `Extend` (`/admin/modules`).

<sup>Напомена!</sup> Модул зависи само од Drupal core-а (`user` и `field`). Не захтева друге Drupal модуле или библиотеке.

Када је модул омогућен, идите у одељак `Configuration` да подесите ваш Tenant ID и API Secret.