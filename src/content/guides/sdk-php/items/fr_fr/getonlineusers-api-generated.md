Actuellement les spectateurs en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés à l'ensemble de la salle, y compris les visiteurs anonymes que nous n'énumérons pas).

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Séparateur en cas d'égalité pour le curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. |

## Réponse

Renvoie : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
$after_name = 'after_name_example'; // string | Curseur : passez nextAfterName depuis la réponse précédente.
$after_user_id = 'after_user_id_example'; // string | Séparateur en cas d'égalité pour le curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]