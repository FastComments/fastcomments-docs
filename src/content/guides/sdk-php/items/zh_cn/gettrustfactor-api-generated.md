---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserTrustFactorResponse.php)

## 示例

[inline-code-attrs-start title = 'getTrustFactor 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果要使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // 字符串
$sso = 'sso_example'; // 字符串

try {
    $result = $apiInstance->getTrustFactor($user_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---