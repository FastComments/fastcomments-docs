### Anforderungen

PHP 7.4 und höher.
Sollte auch mit PHP 8.0 funktionieren.

### Composer

Um die Bindings über [Composer](https://getcomposer.org/) zu installieren, fügen Sie Folgendes zu `composer.json` hinzu:

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

Führen Sie dann `composer install` aus

### Manuelle Installation

Laden Sie die Dateien herunter und binden Sie `autoload.php` ein:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```