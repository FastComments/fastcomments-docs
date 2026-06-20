## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Non |  |
| commentId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserManualBadgesResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getManualBadgesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$badges_user_id = 'badges_user_id_example'; // string
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadgesForUser($badges_user_id, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadgesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]