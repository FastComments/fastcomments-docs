### 要求

PHP 7.4 及更高版本。
也应适用于 PHP 8.0。

### Composer

要通过 [Composer](https://getcomposer.org/) 安装绑定库，请将以下内容添加到 `composer.json`：

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

然后运行 `composer install`

### 手动安装

下载文件并包含 `autoload.php`：

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```