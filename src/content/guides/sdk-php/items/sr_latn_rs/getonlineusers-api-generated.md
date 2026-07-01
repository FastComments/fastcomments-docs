Trenutno‑online gledaoci stranice: ljudi čija websocket sesija je trenutno pretplaćena na stranicu.  
Vraća anonCount + totalCount (pretplatnici na čitavu sobu, uključujući anonimne gledaoce koje ne navodimo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor prekidač: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se izbeglo izostavljanje unosa zbog istih imena. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao podrazumevano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: prosledite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kursor prekidač: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se izbeglo izostavljanje unosa zbog istih imena.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]