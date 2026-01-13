## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModerators200Response.php)

## 示例

[inline-code-attrs-start title = 'getModerators 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 配置 API 密钥授权：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 如有需要，取消下面注释以为 API 密钥设置前缀（例如 Bearer）
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果要使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getModerators($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerators: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---