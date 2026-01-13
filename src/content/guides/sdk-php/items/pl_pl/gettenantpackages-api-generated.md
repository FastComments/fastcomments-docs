## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackages200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantPackages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // Jest to opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantPackages($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantPackages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]