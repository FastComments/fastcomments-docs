### Krav

PHP 7.4 eller senere.
Bør også fungere med PHP 8.0.

### Composer

For at installere bindings via [Composer](https://getcomposer.org/), tilføj følgende til `composer.json`:

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

Kør derefter `composer install`

### Manuel installation

Download filerne og inkluder `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```