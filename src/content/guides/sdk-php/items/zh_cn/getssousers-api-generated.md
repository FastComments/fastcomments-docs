## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | integer | query | 否 |  |

## 响应

返回: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUsers200Response.php)

## 示例

[inline-code-attrs-start title = 'getSSOUsers 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如果需要，请取消下面注释以设置 API 密钥的前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果想使用自定义的 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 56; // int

try {
    $result = $apiInstance->getSSOUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]