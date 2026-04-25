Модуль FastComments для Drupal замінює вбудовані коментарі Drupal на швидку систему коментування в реальному часі. Модуль опублікований на [drupal.org](https://www.drupal.org/project/fcom) і працює з Drupal 10 та 11.

Існує два способи його встановлення.

## Install with Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Install manually

Завантажте модуль з [drupal.org/project/fcom](https://www.drupal.org/project/fcom) і помістіть його в директорію вашого сайту `modules/custom/fastcomments/`. Потім увімкніть його за допомогою `drush en fastcomments` або через адміністративний інтерфейс у розділі `Extend` (`/admin/modules`).

<sup>Примітка!</sup> Модуль залежить лише від ядра Drupal (`user` і `field`). Інші модулі або бібліотеки Drupal не потрібні.

Після увімкнення модуля перейдіть до розділу `Configuration`, щоб налаштувати ваш Tenant ID та API Secret.