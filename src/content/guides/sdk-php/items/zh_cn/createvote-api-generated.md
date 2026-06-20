---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 是 |  |
| direction | string | query | 是 |  |
| userId | string | query | 否 |  |
| anonUserId | string | query | 否 |  |

## 响应

返回: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## 示例

[inline-code-attrs-start title = 'createVote 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 字符串
$comment_id = 'comment_id_example'; // 字符串
$direction = 'direction_example'; // 字符串
$user_id = 'user_id_example'; // 字符串
$anon_user_id = 'anon_user_id_example'; // 字符串

try {
    $result = $apiInstance->createVote($tenant_id, $comment_id, $direction, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---