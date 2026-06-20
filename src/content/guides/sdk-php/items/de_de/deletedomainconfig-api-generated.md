## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domain | string | path | Ja |  |

## Antwort

Gibt zurück: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteDomainConfigResponse.php)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für deleteDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurieren der API-Schlüssel-Authentifizierung: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Kommentieren Sie die folgende Zeile aus, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls erforderlich
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie einen Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird als Standard verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain = 'domain_example'; // string

try {
    $result = $apiInstance->deleteDomainConfig($tenant_id, $domain);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]