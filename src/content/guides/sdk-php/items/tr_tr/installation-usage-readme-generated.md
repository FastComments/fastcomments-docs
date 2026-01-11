### Gereksinimler

PHP 7.4 ve sonrası.
PHP 8.0 ile de çalışmalıdır.

### Composer

Bağlantıları [Composer](https://getcomposer.org/) aracılığıyla yüklemek için `composer.json` dosyanıza aşağıdakileri ekleyin:

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

Sonra `composer install` çalıştırın

### Manuel Kurulum

Dosyaları indirin ve `autoload.php` dosyasını dahil edin:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```