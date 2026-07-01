## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nee |  |

## Respons

Retourneert: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantUsersResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getTenantUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configureer API-sleutel autorisatie: api_key
// Verwijder commentaar hieronder om een prefix in te stellen (bijv. Bearer) voor de API-sleutel, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als je een aangepaste http-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt gebruikt als standaard.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]