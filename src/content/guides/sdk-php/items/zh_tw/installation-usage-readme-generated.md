---
### 需求

PHP 7.4 或更新版本。  
也應可在 PHP 8.0 上運行。

### Composer

要透過 [Composer](https://getcomposer.org/) 安裝這些綁定，請將下列內容新增到 `composer.json`：

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

然後執行 `composer install`

### 手動安裝

下載檔案並包含 `autoload.php`：

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```
---