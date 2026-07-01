## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | integer | query | Ne |  |

## Response

Vrne: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUsersResponse.php)

## Primer

[inline-code-attrs-start title = 'getSSOUsers Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// // Konfigurirajte avtorizacijo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// // Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // // Če želite uporabiti svoj HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // // To je neobvezno, `GuzzleHttp\Client` bo uporabljen kot privzeto.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$skip = 56; // int


try {
    $result = $apiInstance->getSSOUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]