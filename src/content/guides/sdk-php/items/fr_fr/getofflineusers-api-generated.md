Past commenters on the page who are NOT currently online. Sorted by displayName.  
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Membres ».  
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} à partir de afterName en avant via $gt, sans coût $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : transmettez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Critère de départage du curseur : transmettez nextAfterUserId depuis la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de noms ne suppriment pas d'entrées. |

## Response

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifiant d'URL de la page (nettoyé côté serveur).
$options = [
    'after_name' => 'after_name_example', // string | Curseur : transmettez nextAfterName depuis la réponse précédente.
    'after_user_id' => 'after_user_id_example', // string | Critère de départage du curseur : transmettez nextAfterUserId depuis la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de noms ne suppriment pas d'entrées.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---