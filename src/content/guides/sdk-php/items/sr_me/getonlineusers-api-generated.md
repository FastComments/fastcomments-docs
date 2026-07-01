Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

Trenutno online gledatelji stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.  
Vraća anonCount + totalCount (pretplatnici u cijeloj sobi, uključujući anonimne gledatelje koje ne nabrajamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrješavanje neriješenih slučajeva: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako se ne bi izostavili unosi s istim imenom. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao podrazumevano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kursor za razrješavanje neriješenih slučajeva: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako se ne bi izostavili unosi s istim imenom.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]