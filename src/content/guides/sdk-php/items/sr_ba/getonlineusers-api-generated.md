Trenutni gledatelji stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.  
Vraća anonCount + totalCount (pretplatnici cijele sobe, uključujući anonimne gledatelje koje ne enumeriramo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor razrješavač: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako se ne bi izostavile stavke zbog podudaranja imena. |

## Response

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišten kao podrazumevani.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kursor razrješavač: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je postavljen afterName kako se ne bi izostavile stavke zbog podudaranja imena.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]