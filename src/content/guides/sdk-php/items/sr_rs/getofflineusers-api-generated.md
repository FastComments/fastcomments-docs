Past komentatori na stranici koji NISU trenutno online. Sortirani po displayName.  
Koristite ovo nakon što iscrpite /users/online da prikažete sekciju „Članovi“.  
Kursor paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName} indeks od afterName napred putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor razdvajač: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako vezani nazivi ne bi izostavili unose. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, prosledite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, podrazumevano će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: prosledite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kursor razdvajač: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako vezani nazivi ne bi izostavili unose.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]