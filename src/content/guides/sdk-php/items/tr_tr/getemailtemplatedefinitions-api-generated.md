## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplateDefinitionsResponse.php)

## Example

[inline-code-attrs-start title = 'getEmailTemplateDefinitions Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// API anahtarı yetkilendirmesini yapılandır: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırın yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Özel HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi iletin.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string


try {
    $result = $apiInstance->getEmailTemplateDefinitions($tenant_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplateDefinitions: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---