## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| id | string | pot | Da |  |
| updateComments | boolean | poizvedba | Ne |  |

## Odziv

Vrne: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchSSOUserAPIResponse.php)

## Primer

[inline-code-attrs-start title = 'patchSSOUser Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodnjo vrstico, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti svoj HTTP odjemalec, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko; kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_apisso_user_data = new \FastComments\Client\Model\UpdateAPISSOUserData(); // \FastComments\Client\Model\UpdateAPISSOUserData
$update_comments = True; // bool

try {
    $result = $apiInstance->patchSSOUser($tenant_id, $id, $update_apisso_user_data, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]