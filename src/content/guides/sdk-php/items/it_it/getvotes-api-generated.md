## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | Sì |  |

## Risposta

Restituisce: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotes200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getVotes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autenticazione con chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Decommenta la riga sottostante per impostare un prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà utilizzato come `GuzzleHttp\Client` di default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getVotes($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]