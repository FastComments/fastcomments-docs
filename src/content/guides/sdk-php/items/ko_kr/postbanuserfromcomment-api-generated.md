## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| banEmail | boolean | query | 아니요 |  |
| banEmailDomain | boolean | query | 아니요 |  |
| banIP | boolean | query | 아니요 |  |
| deleteAllUsersComments | boolean | query | 아니요 |  |
| bannedUntil | string | query | 아니요 |  |
| isShadowBan | boolean | query | 아니요 |  |
| updateId | string | query | 아니요 |  |
| banReason | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## 예제

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항이며 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
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