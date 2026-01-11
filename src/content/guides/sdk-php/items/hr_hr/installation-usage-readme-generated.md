### Zahtjevi

PHP 7.4 i novije.
Također bi trebalo raditi s PHP 8.0.

### Composer

Za instalaciju bindingsa putem [Composer](https://getcomposer.org/), dodajte sljedeće u `composer.json`:

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

Zatim pokrenite `composer install`

### Ručna instalacija

Preuzmite datoteke i uključite `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```