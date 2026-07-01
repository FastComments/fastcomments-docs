Momenteel-online kijkers van een pagina: mensen waarvan de websocket‑sessie op dit moment op de pagina is geabonneerd.  
Retourneert anonCount + totalCount (abonnees in de hele ruimte, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina‑URL‑identificatie (server‑side opgeschoond). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door van de vorige respons. |
| afterUserId | string | query | Nee | Cursor‑tiebreaker: geef nextAfterUserId door van de vorige respons. Vereist wanneer afterName is ingesteld zodat naam‑gelijkstellingen geen items laten vallen. |

## Response

Retourneert: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als je een aangepaste HTTP‑client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Pagina‑URL‑identificatie (server‑side opgeschoond).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: geef nextAfterName door van de vorige respons.
    'after_user_id' => 'after_user_id_example', // string | Cursor‑tiebreaker: geef nextAfterUserId door van de vorige respons. Vereist wanneer afterName is ingesteld zodat naam‑gelijkstellingen geen items laten vallen.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---