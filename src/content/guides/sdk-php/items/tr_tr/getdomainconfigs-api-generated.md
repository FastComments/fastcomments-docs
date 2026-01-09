## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Response

Döndürür: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfigs200Response.php)

## Örnek

[inline-code-attrs-start title = 'getDomainConfigs Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtar yetkilendirmesini yapılandır: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak amacıyla aşağıdaki satırın yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçirin.
    // Bu isteğe bağlıdır; varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string

try {
    $result = $apiInstance->getDomainConfigs($tenant_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getDomainConfigs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]