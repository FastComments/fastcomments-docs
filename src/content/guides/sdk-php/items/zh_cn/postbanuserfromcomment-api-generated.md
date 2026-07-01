## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| commentId | string | 路径 | 是 |  |
| banEmail | boolean | 查询 | 否 |  |
| banEmailDomain | boolean | 查询 | 否 |  |
| banIP | boolean | 查询 | 否 |  |
| deleteAllUsersComments | boolean | 查询 | 否 |  |
| bannedUntil | string | 查询 | 否 |  |
| isShadowBan | boolean | 查询 | 否 |  |
| updateId | string | 查询 | 否 |  |
| banReason | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## 示例

[inline-code-attrs-start title = 'postBanUserFromComment 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这不是必需的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$options = [
    'ban_email' => True, // bool
    'ban_email_domain' => True, // bool
    'ban_ip' => True, // bool
    'delete_all_users_comments' => True, // bool
    'banned_until' => 'banned_until_example', // string
    'is_shadow_ban' => True, // bool
    'update_id' => 'update_id_example', // string
    'ban_reason' => 'ban_reason_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->postBanUserFromComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]