## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| meta | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenants200Response.php)

## 示例

[inline-code-attrs-start title = 'getTenants 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如果需要，取消注释下面一行以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自定义的 http 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$meta = 'meta_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenants($tenant_id, $meta, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenants: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---