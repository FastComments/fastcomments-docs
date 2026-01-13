## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 响应

返回: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotesForUser200Response.php)

## 示例

[inline-code-attrs-start title = 'getVotesForUser 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// 配置 API 密钥授权：api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 如果需要，请取消注释下面行以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字符串
$url_id = 'url_id_example'; // 字符串
$user_id = 'user_id_example'; // 字符串
$anon_user_id = 'anon_user_id_example'; // 字符串

try {
    $result = $apiInstance->getVotesForUser($tenant_id, $url_id, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]