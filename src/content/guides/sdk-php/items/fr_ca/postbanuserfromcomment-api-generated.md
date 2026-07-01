## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
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

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BanUserFromCommentResult.php)

## Exemple

[inline-code-attrs-start title = 'Exemple postBanUserFromComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // chaîne
$comment_id = 'comment_id_example'; // chaîne
$options = [
    'ban_email' => True, // booléen
    'ban_email_domain' => True, // booléen
    'ban_ip' => True, // booléen
    'delete_all_users_comments' => True, // booléen
    'banned_until' => 'banned_until_example', // chaîne
    'is_shadow_ban' => True, // booléen
    'update_id' => 'update_id_example', // chaîne
    'ban_reason' => 'ban_reason_example', // chaîne
    'sso' => 'sso_example', // chaîne
];


try {
    $result = $apiInstance->postBanUserFromComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postBanUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---