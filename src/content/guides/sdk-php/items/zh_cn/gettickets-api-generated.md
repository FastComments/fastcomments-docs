## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| state | number | query | 否 |  |
| skip | number | query | 否 |  |
| limit | number | query | 否 |  |

## 响应

返回: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTickets200Response.php)

## 示例

[inline-code-attrs-start title = 'getTickets 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果要使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这不是必需的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字符串
$user_id = 'user_id_example'; // 字符串
$state = 3.4; // 浮点数
$skip = 3.4; // 浮点数
$limit = 3.4; // 浮点数

try {
    $result = $apiInstance->getTickets($tenant_id, $user_id, $state, $skip, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]