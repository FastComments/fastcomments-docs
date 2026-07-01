Past komentatori na stranici koji NIJE trenutno online. Sortirani po displayName. Koristite ovo nakon što ste iscrpili /users/online da prikažete sekciju „Members“. Cursor paginacija po commenterName: server prolazi kroz parcijalni {tenantId, urlId, commenterName} indeks od afterName napred putem $gt, bez $skip troška.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL‑a stranice (čišćen na serveru). |
| afterName | string | query | No | Cursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Cursor razrješivač: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi izostavile stavke s istim imenom. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, kao podrazumevani će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL‑a stranice (čišćen na serveru).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: proslijedite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Cursor razrješivač: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se ne bi izostavile stavke s istim imenom.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]