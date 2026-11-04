Модуль FastComments для Drupal заменяет встроенные комментарии Drupal на быструю систему комментирования в реальном времени. Модуль [опубликован на drupal.org](https://www.drupal.org/project/fcom) и работает с Drupal 10 и 11.

Есть два способа установки.

## Установка с помощью Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Установка вручную

Скачайте модуль со страницы [drupal.org/project/fcom](https://www.drupal.org/project/fcom) и поместите его в каталог `modules/custom/fastcomments/` вашего сайта. Затем включите его с помощью `drush en fastcomments` или через административный интерфейс в разделе `Extend` (`/admin/modules`).

<sup>Обратите внимание!</sup> Модуль зависит только от ядра Drupal (`user` и `field`). Другие модули или библиотеки Drupal не требуются.

После включения модуля перейдите в раздел `Configuration`, чтобы настроить ваш Tenant ID и API Secret.