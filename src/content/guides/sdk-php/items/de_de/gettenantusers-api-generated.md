## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|---------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nein |  |

## Antwort

Rückgabe: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantUsersResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getTenantUsers Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriere API-Key-Autorisierung: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie den Kommentar darunter, um ein Präfix (z.B. Bearer) für den API-Key einzurichten, falls nötig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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