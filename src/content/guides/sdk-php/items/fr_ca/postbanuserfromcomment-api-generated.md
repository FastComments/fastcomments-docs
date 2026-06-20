## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| banEmail | boolean | query | Non |  |
| banEmailDomain | boolean | query | Non |  |
| banIP | boolean | query | Non |  |
| deleteAllUsersComments | boolean | query | Non |  |
| bannedUntil | string | query | Non |  |
| isShadowBan | boolean | query | Non |  |
| updateId | string | query | Non |  |
| banReason | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de postBanUserFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
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