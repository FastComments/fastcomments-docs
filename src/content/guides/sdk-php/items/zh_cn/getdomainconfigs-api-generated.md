## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |

## 响应

返回: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfigsResponse.php)

## 示例

[inline-code-attrs-start title = 'getDomainConfigs 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
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