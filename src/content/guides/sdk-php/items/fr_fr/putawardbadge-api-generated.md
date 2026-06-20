## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Oui |  |
| userId | string | query | Non |  |
| commentId | string | query | Non |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AwardUserBadgeResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de putAwardBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$badge_id = 'badge_id_example'; // string
$user_id = 'user_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->putAwardBadge($badge_id, $user_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putAwardBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]