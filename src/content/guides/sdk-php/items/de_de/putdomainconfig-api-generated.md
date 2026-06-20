## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Antwort

Gibt zurück: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PutDomainConfigResponse.php)

## Beispiel

[inline-code-attrs-start title = 'putDomainConfig Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Autorisierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Entfernen Sie das Kommentarzeichen unten, um ein Präfix (z. B. Bearer) für den API-Schlüssel zu setzen, falls erforderlich
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, standardmäßig wird `GuzzleHttp\Client` verwendet.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain_to_update = 'domain_to_update_example'; // string
$update_domain_config_params = new \FastComments\Client\Model\UpdateDomainConfigParams(); // \FastComments\Client\Model\UpdateDomainConfigParams

try {
    $result = $apiInstance->putDomainConfig($tenant_id, $domain_to_update, $update_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->putDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]