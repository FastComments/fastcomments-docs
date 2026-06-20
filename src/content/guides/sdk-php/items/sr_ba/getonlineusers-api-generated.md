Trenutno online posjetioci stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici u okviru sobe, uključujući anonimne posjetioce koje ne nabrajamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (obrađeno na serverskoj strani). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Tajbrejker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen, kako stavke sa istim imenom ne bi bile izostavljene. |

## Response

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadani.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (obrađeno na serverskoj strani).
$after_name = 'after_name_example'; // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
$after_user_id = 'after_user_id_example'; // string | Tajbrejker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen, kako stavke sa istim imenom ne bi bile izostavljene.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]