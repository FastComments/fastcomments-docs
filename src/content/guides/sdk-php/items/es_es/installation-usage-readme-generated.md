### Requisitos

PHP 7.4 y posteriores.
También debería funcionar con PHP 8.0.

### Composer

Para instalar las bibliotecas mediante [Composer](https://getcomposer.org/), añada lo siguiente a `composer.json`:

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

Luego, ejecute `composer install`

### Instalación manual

Descargue los archivos e incluya `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```