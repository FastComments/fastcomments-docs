## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## 响应

返回：[`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogsResponse.php)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 取消注释以下代码以设置 API 密钥的前缀（例如 Bearer），如果需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是一项可选设置，默认使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'limit' => 3.4, // float
    'skip' => 3.4, // float
    'order' => new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(), // \FastComments\Client\Model\SORTDIR
    'after' => 3.4, // float
    'before' => 3.4, // float
];


try {
    $result = $apiInstance->getAuditLogs($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]