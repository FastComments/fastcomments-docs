Belgeleri gruplayarak (groupBy sağlanmışsa) toplar ve birden fazla işlem uygular. Farklı işlemler (ör. sum, countDistinct, avg vb.) desteklenir.

## Parameters

| Ad | Type | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| parentTenantId | string | query | Hayır |  |
| includeStats | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`AggregateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateResponse.php)

## Örnek

[inline-code-attrs-start title = 'aggregate Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandır: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak için aşağıdaki satırı yorumdan çıkarın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcisi kullanmak isterseniz, `GuzzleHttp\ClientInterface` uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$aggregation_request = new \FastComments\Client\Model\AggregationRequest(); // \FastComments\Client\Model\AggregationRequest
$parent_tenant_id = 'parent_tenant_id_example'; // string
$include_stats = True; // bool

try {
    $result = $apiInstance->aggregate($tenant_id, $aggregation_request, $parent_tenant_id, $include_stats);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]