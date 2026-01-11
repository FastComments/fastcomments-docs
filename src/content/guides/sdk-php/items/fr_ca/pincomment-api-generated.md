## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| commentId | string | chemin | Oui |  |
| broadcastId | string | requête | Oui |  |
| sso | string | requête | Non |  |

## Réponse

Retourne: [`PinComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PinComment200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de pinComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->pinComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->pinComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]