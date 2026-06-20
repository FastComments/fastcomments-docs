## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| domainToUpdate | string | path | Sì |  |

## Risposta

Restituisce: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchDomainConfigResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di patchDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autenticazione con API key: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Decommentare la riga sottostante per impostare un prefisso (es. Bearer) per la API key, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client HTTP personalizzato, passa il client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain_to_update = 'domain_to_update_example'; // string
$patch_domain_config_params = new \FastComments\Client\Model\PatchDomainConfigParams(); // \FastComments\Client\Model\PatchDomainConfigParams

try {
    $result = $apiInstance->patchDomainConfig($tenant_id, $domain_to_update, $patch_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]