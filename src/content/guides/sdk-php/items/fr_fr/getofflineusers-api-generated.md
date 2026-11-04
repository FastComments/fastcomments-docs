Anciens commentateurs de la page qui ne sont PAS actuellement en ligne. Triés par displayName.
À utiliser après avoir épuisé /users/online pour afficher une section « Membres ».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName}
à partir de afterName via $gt, sans coût de $skip.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les homonymies ne fassent pas perdre d'entrées. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client http personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifiant d'URL de la page (nettoyé côté serveur).
$options = [
    'after_name' => 'after_name_example', // string | Curseur : passez nextAfterName de la réponse précédente.
    'after_user_id' => 'after_user_id_example', // string | Départage du curseur : passez nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les homonymies ne fassent pas perdre d'entrées.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]