Past komentatori na stranici koji NIJE trenutno online. Sortirano po displayName.  
Koristite ovo nakon što iscrpite /users/online kako biste prikazali odeljak "Members" sekciju.  
Kursor paginacija po commenterName: server prolazi kroz parsial {tenantId, urlId, commenterName}  
indeks od afterName napred putem $gt, bez troška $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na serveru). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Tiebreaker za kursor: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se izbeglo izostavljanje unosa pri vezama imena. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao podrazumevano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Kursor: prosledite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Tiebreaker za kursor: prosledite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako bi se izbeglo izostavljanje unosa pri vezama imena.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]