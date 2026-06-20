Trenutno-online posetioci stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici sobe u celini, uključujući anonimne posetioce koje ne nabrajamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tiebreaker kursora: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen, kako se stavke sa istim imenom ne bi izostavile. |

## Response

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Primer

[inline-code-attrs-start title = 'getOnlineUsers Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni http klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišćen kao podrazumevani.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (očišćen na serverskoj strani).
$after_name = 'after_name_example'; // string | Kursor: prosledite nextAfterName iz prethodnog odgovora.
$after_user_id = 'after_user_id_example'; // string | Tiebreaker kursora: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen, kako se stavke sa istim imenom ne bi izostavile.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]