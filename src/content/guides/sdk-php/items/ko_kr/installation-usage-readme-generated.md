### 요구 사항

PHP 7.4 이상.
또한 PHP 8.0에서도 작동해야 합니다.

### Composer

바인딩을 [Composer](https://getcomposer.org/)를 통해 설치하려면, 다음을 `composer.json`에 추가하세요:

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

그런 다음 `composer install`을 실행하세요

### 수동 설치

파일을 다운로드하고 `autoload.php`를 포함하세요:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```