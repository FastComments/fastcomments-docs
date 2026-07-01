The SDK предоставляет три класса клиентского API:

- **`DefaultApi`** - методы, аутентифицированные по API-ключу, для использования на сервере. Настройте API-ключ как показано в [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - публичные методы, не требующие API-ключа, безопасно вызываемые из браузеров и мобильных приложений.
- **`ModerationApi`** - обширный набор живых и быстрых API модерации. Каждый метод `ModerationApi` принимает параметр `$sso` и может аутентифицироваться через SSO или cookie сессии FastComments.com.

### Использование PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### Использование ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload authenticating the moderator

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```