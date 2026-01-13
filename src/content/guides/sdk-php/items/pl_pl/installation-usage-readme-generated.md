---
### Wymagania

PHP 7.4 lub nowszy.
Powinno również działać z PHP 8.0.

### Composer

Aby zainstalować powiązania za pomocą [Composer](https://getcomposer.org/), dodaj do `composer.json` następujące:

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

Następnie uruchom `composer install`

### Ręczna instalacja

Pobierz pliki i dołącz `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```
---