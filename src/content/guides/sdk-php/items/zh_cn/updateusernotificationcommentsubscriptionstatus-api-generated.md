为特定评论启用或禁用通知。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| notificationId | string | 路径 | 是 |  |
| optedInOrOut | string | 路径 | 是 |  |
| commentId | string | 查询 | 是 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationCommentSubscriptionStatusResponse.php)

## 示例

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果要使用自定义 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 字符串
$notification_id = 'notification_id_example'; // 字符串
$opted_in_or_out = 'opted_in_or_out_example'; // 字符串
$comment_id = 'comment_id_example'; // 字符串
$sso = 'sso_example'; // 字符串


try {
    $result = $apiInstance->updateUserNotificationCommentSubscriptionStatus($tenant_id, $notification_id, $opted_in_or_out, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationCommentSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]