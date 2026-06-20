---
## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| trustFactor | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetUserTrustFactorResponse.php)

## 示例

[inline-code-attrs-start title = 'setTrustFactor 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$trust_factor = 'trust_factor_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->setTrustFactor($user_id, $trust_factor, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->setTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---