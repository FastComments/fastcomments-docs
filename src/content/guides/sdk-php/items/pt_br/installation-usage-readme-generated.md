### Requisitos

PHP 7.4 ou posterior.
Também deve funcionar com PHP 8.0.

### Composer

Para instalar os bindings via [Composer](https://getcomposer.org/), adicione o seguinte em `composer.json`:

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

Em seguida, execute `composer install`

### Instalação Manual

Faça o download dos arquivos e inclua `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```