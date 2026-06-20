The SDK üç API istemci sınıfı sunar:

- **`DefaultApi`** — sunucu tarafı kullanım için API anahtarına dayalı kimlik doğrulaması kullanan yöntemler. API anahtarını [Başlarken](#getting-started-readme-generated) bölümünde gösterildiği gibi yapılandırın.
- **`PublicApi`** — API anahtarı gerektirmeyen genel yöntemler; tarayıcılar ve mobil uygulamalardan çağırmak güvenlidir.
- **`ModerationApi`** — moderatör panosu için yöntemler: yorumları listeleme, sayma, arama, kaydetme ve dışa aktarma; moderasyon eylemleri (kaldır/g geri yükle, işaretleme, inceleme/spam/onay durumunu ayarlama, oylar, konuyu yeniden açma/kapatma); yasaklamalar (yoruma yasaklama, geri alma, ön-yasak özetleri, yasak durumu ve tercihleri, yasaklı kullanıcı sayıları); ve rozetler & güven (rozet verme/kaldırma, manuel rozetler, güven faktörünü al/ayarla, kullanıcının dahili profili). Her `ModerationApi` yöntemi, SSO ile işlem yapan moderatörü doğrulamak için bir `$sso` parametresi kabul eder.

### PublicApi Kullanımı

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Genel yöntemlerin API anahtarına ihtiyacı yoktur.
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
$sso = 'sso_example'; // string - Moderatörü SSO ile doğrulayan SSO yükü

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```