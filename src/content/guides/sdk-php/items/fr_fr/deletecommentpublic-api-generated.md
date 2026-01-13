## Paramètres

| Name | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| commentId | string | chemin | Oui |  |
| broadcastId | string | paramètre de requête | Oui |  |
| editKey | string | paramètre de requête | Non |  |
| sso | string | paramètre de requête | Non |  |

## Réponse

Retourne : [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentPublic200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
$edit_key = 'edit_key_example'; // chaîne
$sso = 'sso_example'; // chaîne

try {
    $result = $apiInstance->deleteCommentPublic($tenant_id, $comment_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]