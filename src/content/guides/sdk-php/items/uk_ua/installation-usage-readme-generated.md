### Вимоги

PHP 7.4 і пізніші версії.
Також має працювати з PHP 8.0.

### Composer

Щоб встановити бібліотеку через [Composer](https://getcomposer.org/), додайте наступне до `composer.json`:

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

Потім виконайте `composer install`

### Ручне встановлення

Завантажте файли та підключіть `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```