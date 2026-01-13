---
### Вимоги

PHP 7.4 and later.
Should also work with PHP 8.0.

### Composer

Щоб встановити bindings за допомогою [Composer](https://getcomposer.org/), додайте наступне до `composer.json`:

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

Then run `composer install`

### Manual Installation

Download the files and include `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```
---