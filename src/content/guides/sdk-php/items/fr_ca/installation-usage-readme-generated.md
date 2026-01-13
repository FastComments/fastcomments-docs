---
### Exigences

PHP 7.4 et versions ultérieures.
Devrait également fonctionner avec PHP 8.0.

### Composer

Pour installer les bindings via [Composer](https://getcomposer.org/), ajoutez ce qui suit à `composer.json` :

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

Puis exécutez `composer install`

### Installation manuelle

Téléchargez les fichiers et incluez `autoload.php` :

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```
---