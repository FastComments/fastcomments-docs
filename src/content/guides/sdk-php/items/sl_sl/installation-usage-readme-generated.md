### Zahteve

PHP 7.4 ali novejši.
Naj bi delovalo tudi s PHP 8.0.

### Composer

Za namestitev vezav s pomočjo [Composer](https://getcomposer.org/), dodajte naslednje v `composer.json`:

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

Nato zaženite `composer install`

### Ročna namestitev

Prenesite datoteke in vključite `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```