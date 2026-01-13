## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantUser200Response.php)

## Örnek

[inline-code-attrs-start title = 'createTenantUser Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandırın: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdaki satırı yorumdan çıkarın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_user_body = new \FastComments\Client\Model\CreateTenantUserBody(); // \FastComments\Client\Model\CreateTenantUserBody

try {
    $result = $apiInstance->createTenantUser($tenant_id, $create_tenant_user_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]