Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Side-URL-identifikator (renset på serveren). |
| afterName | string | query | No | Cursor: send nextAfterName fra den foregående svar. |
| afterUserId | string | query | No | Cursor tiebreaker: send nextAfterUserId fra den foregående svar. Påkrævet når afterName er angivet, så navne‑ties ikke udelader poster. |

## Response

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du videregive din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Side-URL-identifikator (renset på serveren).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: send nextAfterName fra den foregående svar.
    'after_user_id' => 'after_user_id_example', // string | Cursor tiebreaker: send nextAfterUserId fra den foregående svar. Påkrævet når afterName er angivet, så navne‑ties ikke udelader poster.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Undtagelse ved kald af PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]