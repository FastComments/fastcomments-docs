### Requisiti

PHP 7.4 e versioni successive.
Dovrebbe funzionare anche con PHP 8.0.

### Composer

Per installare i binding tramite [Composer](https://getcomposer.org/), aggiungi quanto segue a `composer.json`:

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

Poi esegui `composer install`

### Installazione manuale

Scarica i file e includi `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```