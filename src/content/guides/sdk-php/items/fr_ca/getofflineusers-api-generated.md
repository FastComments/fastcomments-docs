Past commentateurs sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.  
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Membres ».  
Pagination par curseur sur commenterName : le serveur parcourt la partie {tenantId, urlId, commenterName} indice à partir de afterName vers l’avant via $gt, sans coût $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | query | No | Curseur de désambigation : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms ne suppriment pas d'entrées. |

## Response

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifiant d'URL de la page (nettoyé côté serveur).
$options = [
    'after_name' => 'after_name_example', // string | Curseur : transmettre nextAfterName de la réponse précédente.
    'after_user_id' => 'after_user_id_example', // string | Curseur de désambigation : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms ne suppriment pas d'entrées.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]