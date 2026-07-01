## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|-----------|-----------|-------------|
| tenantId | string | query | Yes |  |

## Risposta

Restituisce: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateSubscriptionAPIResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio createSubscription'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autorizzazione della chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Decommenta la riga sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // stringa
$create_api_user_subscription_data = new \FastComments\Client\Model\CreateAPIUserSubscriptionData(); // \FastComments\Client\Model\CreateAPIUserSubscriptionData


try {
    $result = $apiInstance->createSubscription($tenant_id, $create_api_user_subscription_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]