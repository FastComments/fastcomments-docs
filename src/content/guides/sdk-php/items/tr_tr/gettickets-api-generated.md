## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| state | number | query | Hayır |  |
| skip | number | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTickets200Response.php)

## Örnek

[inline-code-attrs-start title = 'getTickets Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandır: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcusu kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır; varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$state = 3.4; // float
$skip = 3.4; // float
$limit = 3.4; // float

try {
    $result = $apiInstance->getTickets($tenant_id, $user_id, $state, $skip, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]