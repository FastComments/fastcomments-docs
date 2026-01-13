### Vereisten

PHP 7.4 en later.
Zou ook moeten werken met PHP 8.0.

### Composer

Om de bindings via [Composer](https://getcomposer.org/) te installeren, voeg het volgende toe aan `composer.json`:

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

Voer vervolgens `composer install` uit

### Handmatige installatie

Download de bestanden en voeg `autoload.php` toe:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```