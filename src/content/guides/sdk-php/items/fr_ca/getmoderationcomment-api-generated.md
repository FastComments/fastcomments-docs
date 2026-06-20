## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | chemin | Oui |  |
| includeEmail | boolean | requête | Non |  |
| includeIP | boolean | requête | Non |  |
| sso | string | requête | Non |  |

## Réponse

Retourne : [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getModerationComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // chaîne
$include_email = True; // booléen
$include_ip = True; // booléen
$sso = 'sso_example'; // chaîne

try {
    $result = $apiInstance->getModerationComment($comment_id, $include_email, $include_ip, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]