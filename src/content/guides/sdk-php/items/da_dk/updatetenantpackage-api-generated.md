## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|-----------|--------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |

## Svar

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Eksempel

[inline-code-attrs-start title = 'updateTenantPackage Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøglegodkendelse: api_key
// Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // streng
$id = 'id_example'; // streng
$update_tenant_package_body = new \FastComments\Client\Model\UpdateTenantPackageBody(); // \FastComments\Client\Model\UpdateTenantPackageBody


try {
    $result = $apiInstance->updateTenantPackage($tenant_id, $id, $update_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---