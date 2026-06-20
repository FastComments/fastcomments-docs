## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Antwort

Gibt zurück: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantPackageResponse.php)

## Beispiel

[inline-code-attrs-start title = 'createTenantPackage Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Authentifizierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie unten den Kommentar, um ein Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls benötigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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