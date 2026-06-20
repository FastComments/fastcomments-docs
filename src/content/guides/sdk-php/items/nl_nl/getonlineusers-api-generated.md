Op dit moment online kijkers van een pagina: personen wiens websocket-sessie op dit moment op de pagina geabonneerd is.
Geeft anonCount + totalCount terug (kamerbrede abonnees, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifier van de pagina-URL (door de server opgeschoond). |
| afterName | string | query | No | Cursor: geef nextAfterName uit de vorige respons door. |
| afterUserId | string | query | No | Tiebreaker voor cursor: geef nextAfterUserId uit de vorige respons door. Vereist wanneer afterName is ingesteld, zodat bij gelijke namen geen vermeldingen verloren gaan. |

## Antwoord

Geeft terug: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt gebruikt als standaard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifier van de pagina-URL (door de server opgeschoond).
$after_name = 'after_name_example'; // string | Cursor: geef nextAfterName uit de vorige respons door.
$after_user_id = 'after_user_id_example'; // string | Cursor tiebreaker: geef nextAfterUserId uit de vorige respons door. Vereist wanneer afterName is ingesteld, zodat bij gelijke namen geen vermeldingen verloren gaan.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]