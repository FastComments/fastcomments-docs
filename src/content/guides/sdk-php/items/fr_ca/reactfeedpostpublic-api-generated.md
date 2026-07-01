## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|--------------|-------------|
| tenantId | string | path | Oui |  |
| postId | string | path | Oui |  |
| isUndo | boolean | query | Non |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ReactFeedPostResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple reactFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$react_body_params = new \FastComments\Client\Model\ReactBodyParams(); // \FastComments\Client\Model\ReactBodyParams
$options = [
    'is_undo' => True, // bool
    'broadcast_id' => 'broadcast_id_example', // chaîne
    'sso' => 'sso_example', // chaîne
];


try {
    $result = $apiInstance->reactFeedPostPublic($tenant_id, $post_id, $react_body_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->reactFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]