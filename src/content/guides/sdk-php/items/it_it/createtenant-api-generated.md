## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di createTenant'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configura l'autenticazione della chiave API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato come predefinito.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_body = new \FastComments\Client\Model\CreateTenantBody(); // \FastComments\Client\Model\CreateTenantBody

try {
    $result = $apiInstance->createTenant($tenant_id, $create_tenant_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenant: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]