## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Response

Restituisce: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantsResponse.php)

## Example

[inline-code-attrs-start title = 'Esempio getTenants'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configura l'autorizzazione della chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Decommenta qui sotto per impostare il prefisso (ad esempio Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se desideri utilizzare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come default.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'meta' => 'meta_example', // string
    'skip' => 3.4, // float
];


try {
    $result = $apiInstance->getTenants($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Eccezione durante la chiamata DefaultApi->getTenants: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]