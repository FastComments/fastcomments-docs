## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nej |  |
| banEmailDomain | boolean | query | Nej |  |
| banIP | boolean | query | Nej |  |
| deleteAllUsersComments | boolean | query | Nej |  |
| bannedUntil | string | query | Nej |  |
| isShadowBan | boolean | query | Nej |  |
| updateId | string | query | Nej |  |
| banReason | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Eksempel

[inline-code-attrs-start title = 'postBanUserFromComment Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en tilpasset HTTP-klient, giv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
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