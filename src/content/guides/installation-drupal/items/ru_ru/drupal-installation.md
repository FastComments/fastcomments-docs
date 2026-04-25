---
Модуль FastComments для Drupal заменяет встроенные комментарии Drupal на быструю систему комментариев в реальном времени. Модуль [опубликован на drupal.org](https://www.drupal.org/project/fcom) и совместим с Drupal 10 и 11.

Есть два способа установки.

## Установка с помощью Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Установка вручную

Download the module from [drupal.org/project/fcom](https://www.drupal.org/project/fcom) and place it in your site's `modules/custom/fastcomments/` directory. Then enable it with `drush en fastcomments`, or from the admin UI at `Extend` (`/admin/modules`).

<sup>Примечание!</sup> Модуль зависит только от ядра Drupal (`user` и `field`). Другие модули Drupal или библиотеки не требуются.

После включения модуля перейдите в раздел `Configuration` для настройки Tenant ID и API Secret.

---