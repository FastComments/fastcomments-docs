### Захтеви

PHP 7.4 и новији.
Требало би да ради и са PHP 8.0.

### Composer

Да бисте инсталирали пакете преко [Composer](https://getcomposer.org/), додајте следеће у `composer.json`:

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

Затим покрените `composer install`

### Ручна инсталација

Преузмите фајлове и укључите `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```