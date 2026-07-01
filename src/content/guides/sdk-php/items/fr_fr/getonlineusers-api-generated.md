Currently‑online viewers of a page : people whose websocket session is subscribed to the page right now.  
Renvoie anonCount + totalCount (room‑wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d’URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Désemploi du curseur : passez nextAfterUserId de la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées. |

## Response

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifiant d’URL de la page (nettoyé côté serveur).
$options = [
    'after_name' => 'after_name_example', // string | Curseur : passez nextAfterName de la réponse précédente.
    'after_user_id' => 'after_user_id_example', // string | Désemploi du curseur : passez nextAfterUserId de la réponse précédente. Obligatoire lorsque afterName est défini afin que les égalités de nom ne suppriment pas d’entrées.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]