### Изисквания

PHP 7.4 и по-нови.
Трябва да работи и с PHP 8.0.

### Composer

За да инсталирате пакета чрез [Composer](https://getcomposer.org/), добавете следното към `composer.json`:

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

След това изпълнете `composer install`

### Ръчна инсталация

Изтеглете файловете и включете `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```