## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
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

## Reactie

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserFromComment Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als je een aangepaste http-client wilt gebruiken, geef dan je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
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