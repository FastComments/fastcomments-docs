## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| domainToUpdate | string | path | Ja |  |

## Svar

Returnerer: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchDomainConfigResponse.php)

## Eksempel

[inline-code-attrs-start title = 'patchDomainConfig Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, overgiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
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