## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| domain | string | path | Da |  |

## Odgovor

Vraća: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteDomainConfig200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer deleteDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfiguriši autorizaciju API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Otkomentariši ispod da podesiš prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišćen podrazumevano.
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

---