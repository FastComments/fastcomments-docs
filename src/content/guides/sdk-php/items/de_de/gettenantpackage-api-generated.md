## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|------|--------------|--------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Antwort

Rückgabe: [`GetTenantPackageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackageResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getTenantPackage Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurieren Sie die API-Schlüsselauthorisierung: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Kommentieren Sie die Zeile unten aus, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel festzulegen
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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