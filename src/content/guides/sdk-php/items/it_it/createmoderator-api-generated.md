## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateModerator200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di createModerator'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configura l'autenticazione tramite API key: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Rimuovi il commento qui sotto per impostare un prefisso (es. Bearer) per la API key, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` di default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_moderator_body = new \FastComments\Client\Model\CreateModeratorBody(); // \FastComments\Client\Model\CreateModeratorBody

try {
    $result = $apiInstance->createModerator($tenant_id, $create_moderator_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]