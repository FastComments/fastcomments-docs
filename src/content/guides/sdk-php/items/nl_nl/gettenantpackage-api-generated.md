## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Response

Retourneert: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackageResponse.php)

## Example

[inline-code-attrs-start title = 'getTenantPackage Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutautorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Verwijder de commentaartekens hieronder om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Dit is optioneel, `GuzzleHttp\Client` zal als standaard worden gebruikt.
    // Als je een aangepaste http-client wilt gebruiken, geef dan je client door die `GuzzleHttp\ClientInterface` implementeert.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->getTenantPackage($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]