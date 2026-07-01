Trenutno online gledatelji stranice: ljudi čija websocket sesija je trenutno pretplaćena na stranicu. Vraća anonCount + totalCount (pretplatnici u celokupnoj sobi, uključujući anonimne gledaoce koje ne nabrajamo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor razrešavač vezanosti: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi izostavile stavke zbog istih imena. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, prosledite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao podrazumevani.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: prosledite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kursor razrešavač vezanosti: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi izostavile stavke zbog istih imena.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---