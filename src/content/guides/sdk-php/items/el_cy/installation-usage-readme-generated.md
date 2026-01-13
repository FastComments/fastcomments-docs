### Απαιτήσεις

PHP 7.4 και νεότερες.
Θα πρέπει επίσης να λειτουργεί με PHP 8.0.

### Composer

Για να εγκαταστήσετε τα bindings μέσω [Composer](https://getcomposer.org/), προσθέστε τα παρακάτω στο `composer.json`:

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

Στη συνέχεια τρέξτε `composer install`

### Χειροκίνητη Εγκατάσταση

Κατεβάστε τα αρχεία και συμπεριλάβετε το `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```