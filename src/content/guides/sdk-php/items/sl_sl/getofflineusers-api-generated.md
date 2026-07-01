Past komentatorji na strani, KI NI trenutno povezani. Razvrščeno po displayName.  
Uporabite to po izčrpanju /users/online za prikaz sekcije "Members".  
Kazalno straničenje na commenterName: strežnik hodi po delnem {tenantId, urlId, commenterName} indeksu od afterName naprej prek $gt, brez $skip stroška.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL strani (čiščen na strežniku). |
| afterName | string | query | No | Kazalec: podajte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kazalec za razreševanje neenakosti: podajte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je afterName nastavljen, da se ne izpustijo vnosi pri enakih imenih. |

## Response

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Example

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojeni HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, `GuzzleHttp\Client` bo uporabljen privzeto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL strani (čiščen na strežniku).
$options = [
    'after_name' => 'after_name_example', // string | Kazalec: podajte nextAfterName iz prejšnjega odgovora.
    'after_user_id' => 'after_user_id_example', // string | Kazalec za razreševanje neenakosti: podajte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je afterName nastavljen, da se ne izpustijo vnosi pri enakih imenah.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]