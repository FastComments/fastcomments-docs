---
Prethodni komentatori na stranici koji NISU trenutno online. Sortirani po displayName.  
Koristite ovo nakon što iscrpite /users/online za prikaz odjeljka "Članovi".  
Paginacija pokazivača po commenterName: poslužitelj prolazi kroz djelomični {tenantId, urlId, commenterName} indeks od afterName naprijed putem $gt, bez troška $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na poslužitelju). |
| afterName | string | query | No | Cursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Cursor tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako se izjednačenja po imenu ne bi izgubila unosi. |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL-a stranice (čišćen na poslužitelju).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: proslijedite nextAfterName iz prethodnog odgovora.
    'after_user_id' => 'after_user_id_example', // string | Cursor tiebreaker: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je postavljen afterName kako se izjednačenja po imenu ne bi izgubila unosi.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---