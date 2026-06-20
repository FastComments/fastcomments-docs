## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 响应

返回: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateModeratorResponse.php)

## 示例

[inline-code-attrs-start title = 'createModerator 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如果需要，取消注释下面以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_moderator_body = new \FastComments\Client\Model\CreateModeratorBody(); // \FastComments\Client\Model\CreateModeratorBody

try {
    $result = $apiInstance->createModerator($tenant_id, $create_moderator_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]