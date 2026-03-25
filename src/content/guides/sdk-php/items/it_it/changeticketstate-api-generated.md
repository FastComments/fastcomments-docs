## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| userId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeTicketState200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di changeTicketState'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Configura l'autenticazione tramite API key: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Decommenta sotto per impostare il prefisso (es. Bearer) per la API key, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$id = 'id_example'; // string
$change_ticket_state_body = new \FastComments\Client\Model\ChangeTicketStateBody(); // \FastComments\Client\Model\ChangeTicketStateBody

try {
    $result = $apiInstance->changeTicketState($tenant_id, $user_id, $id, $change_ticket_state_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->changeTicketState: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---