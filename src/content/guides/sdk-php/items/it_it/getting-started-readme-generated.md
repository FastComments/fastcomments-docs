---
Si prega di seguire la [procedura di installazione](#installation--usage) e poi eseguire quanto segue:

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Configure API key authorization: api_key
// Configura l'autorizzazione della chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Decommenta qui sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato di default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // stringa
$add_domain_config_params = new \FastComments\Client\Model\AddDomainConfigParams(); // \FastComments\Client\Model\AddDomainConfigParams

try {
    $result = $apiInstance->addDomainConfig($tenant_id, $add_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addDomainConfig: ', $e->getMessage(), PHP_EOL;
}

```
---