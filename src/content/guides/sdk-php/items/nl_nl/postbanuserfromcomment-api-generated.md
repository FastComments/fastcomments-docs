## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nee |  |
| banEmailDomain | boolean | query | Nee |  |
| banIP | boolean | query | Nee |  |
| deleteAllUsersComments | boolean | query | Nee |  |
| bannedUntil | string | query | Nee |  |
| isShadowBan | boolean | query | Nee |  |
| updateId | string | query | Nee |  |
| banReason | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserFromComment Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een aangepaste http-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` zal als standaard worden gebruikt.
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