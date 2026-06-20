## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantPackageResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład createTenantPackage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli jest potrzebny
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // To opcjonalne, jako domyślny zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_package_body = new \FastComments\Client\Model\CreateTenantPackageBody(); // \FastComments\Client\Model\CreateTenantPackageBody

try {
    $result = $apiInstance->createTenantPackage($tenant_id, $create_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]