## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentIds | string | query | Oui | Une liste d'identifiants de commentaires séparés par des virgules. |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CheckedCommentsForBlocked200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de checkedCommentsForBlocked'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_ids = 'comment_ids_example'; // string | Une liste d'identifiants de commentaires séparés par des virgules.
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->checkedCommentsForBlocked($tenant_id, $comment_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->checkedCommentsForBlocked: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]