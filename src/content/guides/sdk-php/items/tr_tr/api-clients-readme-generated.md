SDK, üç API istemci sınıfı sunar:

- **`DefaultApi`** - Sunucu tarafı kullanım için API anahtarı ile kimlik doğrulaması yapan metotlar. Bir API anahtarını [Getting Started](#getting-started-readme-generated) bölümünde gösterildiği gibi yapılandırın.
- **`PublicApi`** - API anahtarı gerektirmeyen, tarayıcılardan ve mobil uygulamalardan çağrılması güvenli genel metotlar.
- **`ModerationApi`** - Canlı ve hızlı moderasyon API'lerinden oluşan kapsamlı bir paket. Her `ModerationApi` metodu bir `$sso` parametresi kabul eder ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulaması yapabilir.

### PublicApi Kullanımı

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public metotları bir API anahtarı gerektirmez.
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

### ModerationApi Kullanımı

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - moderatörün kimliğini doğrulayan SSO yükü

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```