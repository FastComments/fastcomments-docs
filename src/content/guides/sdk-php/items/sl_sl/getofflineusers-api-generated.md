Prejšnji komentatorji na strani, ki trenutno NISO v spletu. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online za prikaz razdelka "Člani".
Paginacija s kurzorjem po commenterName: strežnik prehodi parcialni indeks {tenantId, urlId, commenterName} od afterName naprej preko $gt, brez stroška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL strani (počiščen na strežniku). |
| afterName | string | query | No | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Razrešitev neodločenosti kurzorja: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je afterName nastavljen, da vnosi z enakimi imeni ne bodo izpuščeni. |

## Odziv

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Primer

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno; kot privzeti bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Identifikator URL strani (počiščen na strežniku).
$after_name = 'after_name_example'; // string | Kazalec: posredujte nextAfterName iz prejšnjega odgovora.
$after_user_id = 'after_user_id_example'; // string | Razrešitev neodločenosti kurzorja: posredujte nextAfterUserId iz prejšnjega odgovora. Obvezno, ko je afterName nastavljen, da vnosi z enakimi imeni ne bodo izpuščeni.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---