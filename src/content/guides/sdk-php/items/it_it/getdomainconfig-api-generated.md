## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## Risposta

Restituisce: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfig200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configura l'autorizzazione della chiave API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Decommentare la riga seguente per impostare il prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Se vuoi usare un client HTTP personalizzato, passa il client che implementa `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come impostazione predefinita.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain = 'domain_example'; // string

try {
    $result = $apiInstance->getDomainConfig($tenant_id, $domain);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]