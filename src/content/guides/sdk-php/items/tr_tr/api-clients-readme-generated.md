SDK üç API istemci sınıfı sunar:

- **`DefaultApi`** – Sunucu tarafı kullanım için API‑anahtarıyla kimlik doğrulamalı yöntemler. API anahtarını, [Getting Started](#getting-started-readme-generated) bölümünde gösterildiği gibi yapılandırın.
- **`PublicApi`** – API anahtarı gerektirmeyen, tarayıcılardan ve mobil uygulamalardan güvenle çağrılabilen genel yöntemler.
- **`ModerationApi`** – Canlı ve hızlı denetim API'lerinin kapsamlı bir paketi. Her `ModerationApi` yöntemi bir `$sso` parametresi alır ve SSO ya da FastComments.com oturum çerezi aracılığıyla kimlik doğrulayabilir.

### Using PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Genel yöntemler bir API anahtarı gerektirmez.
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

### Using ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - Moderatörü kimlik doğrulayan SSO yükü

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```