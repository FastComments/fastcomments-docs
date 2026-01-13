### Zahtevi

PHP 7.4 i noviji.
Takođe bi trebalo da radi sa PHP 8.0.

### Composer

Da biste instalirali bindinge putem [Composer](https://getcomposer.org/), dodajte sledeće u `composer.json`:

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

### Ručno instaliranje

Preuzmite datoteke i uključite `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```