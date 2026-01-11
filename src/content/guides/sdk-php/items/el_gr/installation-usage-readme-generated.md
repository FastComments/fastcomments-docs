### Απαιτήσεις

PHP 7.4 και νεότερες εκδόσεις.
Θα πρέπει επίσης να λειτουργεί με PHP 8.0.

### Composer

Για να εγκαταστήσετε τις βιβλιοθήκες (bindings) μέσω του [Composer](https://getcomposer.org/), προσθέστε τα ακόλουθα στο `composer.json`:

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

Στη συνέχεια εκτελέστε `composer install`

### Μη αυτόματη εγκατάσταση

Κατεβάστε τα αρχεία και συμπεριλάβετε το `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```