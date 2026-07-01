Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

聚合文档通过对其进行分组（如果提供了 groupBy）并应用多个操作来实现。  
支持不同的操作（例如 sum、countDistinct、avg 等）。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| parentTenantId | string | query | 否 |  |
| includeStats | boolean | query | 否 |  |

## 响应

返回：[`AggregateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregateResponse.php)

## 示例

[inline-code-attrs-start title = '聚合 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 配置 API 密钥授权：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 取消注释以下代码以为 API 密钥设置前缀（例如 Bearer），如有需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');

// 如果您想使用自定义 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
// 这是可选的，默认将使用 `GuzzleHttp\Client`。
$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$aggregation_request = new \FastComments\Client\Model\AggregationRequest(); // \FastComments\Client\Model\AggregationRequest
$options = [
    'parent_tenant_id' => 'parent_tenant_id_example', // string
    'include_stats' => True, // bool
];

try {
    $result = $apiInstance->aggregate($tenant_id, $aggregation_request, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]