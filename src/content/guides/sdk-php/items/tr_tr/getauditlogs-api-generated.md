## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Yanıt

Döner: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogsResponse.php)

## Örnek

[inline-code-attrs-start title = 'getAuditLogs Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandır: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Aşağıdaki satırı açarak API anahtarı için ön ek (ör. Bearer) ayarlayabilirsiniz, gerekirse
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'limit' => 3.4, // float
    'skip' => 3.4, // float
    'order' => new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(), // \FastComments\Client\Model\SORTDIR
    'after' => 3.4, // float
    'before' => 3.4, // float
];


try {
    $result = $apiInstance->getAuditLogs($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---