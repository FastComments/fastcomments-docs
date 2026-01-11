### Требования

PHP 7.4 и выше.  
Также должно работать с PHP 8.0.

### Composer

Чтобы установить привязки через [Composer](https://getcomposer.org/), добавьте следующее в `composer.json`:

```json
{
  "repositories": [
    {
      "type": "vcs",
      "url": "https://github.com/fastcomments/fastcomments-php.git"
    }
  ],
  "require": {
    "fastcomments/fastcomments-php": "*@dev"
  }
}
```

Затем выполните `composer install`

### Ручная установка

Скачайте файлы и подключите `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```