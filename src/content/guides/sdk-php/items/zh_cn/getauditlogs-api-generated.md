## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| limit | number | query | 否 |  |
| skip | number | query | 否 |  |
| order | string | query | 否 |  |
| after | number | query | 否 |  |
| before | number | query | 否 |  |

## 响应

返回: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogs200Response.php)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权：api_key
// 如果需要，请取消下面的注释以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字符串
$limit = 3.4; // 浮点数
$skip = 3.4; // 浮点数
$order = new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(); // \FastComments\Client\Model\SORTDIR
$after = 3.4; // 浮点数
$before = 3.4; // 浮点数

try {
    $result = $apiInstance->getAuditLogs($tenant_id, $limit, $skip, $order, $after, $before);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]