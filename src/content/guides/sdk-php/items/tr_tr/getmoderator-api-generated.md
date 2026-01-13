## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Döndürür: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModerator200Response.php)

## Örnek

[inline-code-attrs-start title = 'getModerator Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandır: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse, aşağıdaki satırın yorumunu kaldırarak API anahtarı için öneki (ör. Bearer) ayarlayın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi iletiniz.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getModerator($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]