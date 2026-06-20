---
Prethodni komentatori na stranici koji trenutno nisu online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da prikažete sekciju "Članovi".
Kursor paginacija po commenterName: server prolazi kroz djelimični {tenantId, urlId, commenterName}
indeks od afterName naprijed koristeći $gt, bez troška $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (obrađen na serveru). |
| afterName | string | query | No | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor tiebreaker: pošaljite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako podjednakosti imena ne bi izostavile unose. |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'getOfflineUsers Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti sopstveni HTTP klijent, proslijedite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao podrazumijevani će biti korišćen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL stranice (obrađen na serveru).
$after_name = 'after_name_example'; // string | Kursor: pošaljite nextAfterName iz prethodnog odgovora.
$after_user_id = 'after_user_id_example'; // string | Kursor tiebreaker: pošaljite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako podjednakosti imena ne bi izostavile unose.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---