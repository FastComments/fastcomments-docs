Модуль FastComments для Drupal замінює вбудовані коментарі Drupal на швидку систему коментування в реальному часі. Модуль [опубліковано на drupal.org](https://www.drupal.org/project/fcom) і працює з Drupal 10 та 11.

Є два способи встановлення.

## Встановлення через Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Встановлення вручну

Завантажте модуль зі сторінки [drupal.org/project/fcom](https://www.drupal.org/project/fcom) і розмістіть його в каталозі `modules/custom/fastcomments/` вашого сайту. Потім увімкніть його за допомогою `drush en fastcomments` або через адміністративний інтерфейс у розділі `Extend` (`/admin/modules`).

<sup>Зверніть увагу!</sup> Модуль залежить лише від ядра Drupal (`user` та `field`). Інші модулі Drupal або бібліотеки не потрібні.

Після увімкнення модуля перейдіть до розділу `Configuration`, щоб налаштувати ваш Tenant ID та API Secret.