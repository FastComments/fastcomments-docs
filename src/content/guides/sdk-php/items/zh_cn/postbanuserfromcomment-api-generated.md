## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| banEmail | boolean | query | 否 |  |
| banEmailDomain | boolean | query | 否 |  |
| banIP | boolean | query | 否 |  |
| deleteAllUsersComments | boolean | query | 否 |  |
| bannedUntil | string | query | 否 |  |
| isShadowBan | boolean | query | 否 |  |
| updateId | string | query | 否 |  |
| banReason | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## 示例

[inline-code-attrs-start title = 'postBanUserFromComment 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$ban_email = True; // bool
$ban_email_domain = True; // bool
$ban_ip = True; // bool
$delete_all_users_comments = True; // bool
$banned_until = 'banned_until_example'; // string
$is_shadow_ban = True; // bool
$update_id = 'update_id_example'; // string
$ban_reason = 'ban_reason_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postBanUserFromComment($comment_id, $ban_email, $ban_email_domain, $ban_ip, $delete_all_users_comments, $banned_until, $is_shadow_ban, $update_id, $ban_reason, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]