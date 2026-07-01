## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|------------|--------------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Esempio

[inline-code-attrs-start title = 'deleteEmailTemplate Esempio'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autorizzazione della chiave API: api_key
// Decommenta qui sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se desideri utilizzare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà utilizzato come predefinito.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->deleteEmailTemplate($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]