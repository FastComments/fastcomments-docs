## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| domain | string | path | Da |  |

## Response

Vrne: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfigResponse.php)

## Example

[inline-code-attrs-start title = 'Primer getDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Nastavite avtentikacijo ključev API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Odkomentirajte spodaj, če je potrebno nastaviti predpono (npr. Bearer) za ključ API
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Če želite uporabiti svoj HTTP odjemalec, podajte svojega odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // To je neobvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// niz
$domain = 'domain_example'; // string
// niz


try {
    $result = $apiInstance->getDomainConfig($tenant_id, $domain);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]