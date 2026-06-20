## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 示例

[inline-code-attrs-start title = 'postBanUserUndo 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$ban_user_undo_params = new \FastComments\Client\Model\BanUserUndoParams(); // \FastComments\Client\Model\BanUserUndoParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBanUserUndo($ban_user_undo_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserUndo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---