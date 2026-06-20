## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| badgeId | string | query | 是 |  |
| userId | string | query | 否 |  |
| commentId | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RemoveUserBadgeResponse.php)

## 示例

[inline-code-attrs-start title = 'putRemoveBadge 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果你想使用自定义的 HTTP 客户端，传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // 字符串
$user_id = 'user_id_example'; // 字符串
$comment_id = 'comment_id_example'; // 字符串
$broadcast_id = 'broadcast_id_example'; // 字符串
$sso = 'sso_example'; // 字符串

try {
    $result = $apiInstance->putRemoveBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putRemoveBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]