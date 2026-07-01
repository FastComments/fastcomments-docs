Currently-online viewers of a page : personnes dont la session websocket est abonnée à la page en ce moment.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d’URL de page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Curseur de résolution d’égalité : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifiant d'URL de page (nettoyé côté serveur).
$options = [
    'after_name' => 'after_name_example', // string | Curseur : transmettre nextAfterName de la réponse précédente.
    'after_user_id' => 'after_user_id_example', // string | Curseur de résolution d'égalité : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom ne suppriment pas d'entrées.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]