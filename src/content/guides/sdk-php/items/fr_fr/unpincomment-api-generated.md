## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| broadcastId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeCommentPinStatusResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'unPinComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // chaîne
$comment_id = 'comment_id_example'; // chaîne
$broadcast_id = 'broadcast_id_example'; // chaîne
$sso = 'sso_example'; // chaîne

try {
    $result = $apiInstance->unPinComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->unPinComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]