The SDK предоставляет три класса API‑клиентов:

- **`DefaultApi`** – методы, требующие аутентификации API‑ключом, предназначены для серверной стороны. Настройте API‑ключ, как показано в разделе [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** – публичные методы, не требующие API‑ключа, безопасные для вызова из браузеров и мобильных приложений.
- **`ModerationApi`** – обширный набор быстрых и живых модерационных API. Каждый метод `ModerationApi` принимает параметр `$sso` и может аутентифицироваться через SSO или cookie‑сеанс FastComments.com.

### Использование PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Публичные методы не требуют API‑ключа.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$url_id = 'url_id_example'; // строка

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
$sso = 'sso_example'; // строка – SSO‑полезная нагрузка, аутентифицирующая модератора

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```