## パラメーター

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| banEmail | boolean | query | いいえ |  |
| banEmailDomain | boolean | query | いいえ |  |
| banIP | boolean | query | いいえ |  |
| deleteAllUsersComments | boolean | query | いいえ |  |
| bannedUntil | string | query | いいえ |  |
| isShadowBan | boolean | query | いいえ |  |
| updateId | string | query | いいえ |  |
| banReason | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## 応答

戻り値: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## 例

[inline-code-attrs-start title = 'postBanUserFromComment 例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションで、デフォルトとして `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 文字列
$comment_id = 'comment_id_example'; // 文字列
$options = [
    'ban_email' => True, // ブール
    'ban_email_domain' => True, // ブール
    'ban_ip' => True, // ブール
    'delete_all_users_comments' => True, // ブール
    'banned_until' => 'banned_until_example', // 文字列
    'is_shadow_ban' => True, // ブール
    'update_id' => 'update_id_example', // 文字列
    'ban_reason' => 'ban_reason_example', // 文字列
    'sso' => 'sso_example', // 文字列
];


try {
    $result = $apiInstance->postBanUserFromComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]