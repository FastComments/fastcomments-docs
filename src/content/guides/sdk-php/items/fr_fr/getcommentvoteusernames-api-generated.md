## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| dir | integer | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentVoteUserNamesSuccessResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentVoteUserNames'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Cela est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$dir = 56; // int
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentVoteUserNames($tenant_id, $comment_id, $dir, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentVoteUserNames: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]