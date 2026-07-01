The SDK‑то предоставя три класа за API клиент:

- **`DefaultApi`** – методи, удостоверени с API‑ключ, за сървърна страна. Конфигурирайте API‑ключ, както е показано в [Започване](#getting-started-readme-generated).
- **`PublicApi`** – публични методи, които не изискват API‑ключ, безопасни за повикване от браузъри и мобилни приложения.
- **`ModerationApi`** – обширен набор от живи и бързи API за модериране. Всеки метод на `ModerationApi` приема параметър `$sso` и може да се удостоверява чрез SSO или с FastComments.com сесийна бисквитка.

### Използване на PublicApi

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

### Използване на ModerationApi

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