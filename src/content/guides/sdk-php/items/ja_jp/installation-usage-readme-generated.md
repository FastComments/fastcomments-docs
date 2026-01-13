---
### 要件

PHP 7.4 以降。
PHP 8.0 でも動作するはずです。

### Composer

バインディングを [Composer](https://getcomposer.org/) 経由でインストールするには、次を `composer.json` に追加してください:

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

その後、`composer install` を実行してください

### 手動インストール

ファイルをダウンロードして `autoload.php` を読み込んでください:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```
---