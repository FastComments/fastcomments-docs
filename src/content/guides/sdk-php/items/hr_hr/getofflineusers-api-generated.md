Prošli komentatori na stranici koji TRENUTNO NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online kako biste prikazali odjeljak "Članovi".
Straničenje s kursorom po commenterName: poslužitelj prolazi parcijalni {tenantId, urlId, commenterName} index od afterName unaprijed putem $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (očišćen na strani poslužitelja). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrješenje izjednačenja: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo izbacivanje zapisa pri istim imenima. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao zadani će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL stranice (očišćen na strani poslužitelja).
$after_name = 'after_name_example'; // string | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
$after_user_id = 'after_user_id_example'; // string | Kursor za razrješenje izjednačenja: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo izbacivanje zapisa pri istim imenima.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---