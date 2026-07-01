## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Risposta

Restituisce: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotesForUserResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getVotesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Configura l'autorizzazione della chiave API: api_key
// Decommenta qui sotto per impostare il prefisso (ad es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // stringa
$url_id = 'url_id_example'; // stringa
$options = [
    'user_id' => 'user_id_example', // stringa
    'anon_user_id' => 'anon_user_id_example', // stringa
];


try {
    $result = $apiInstance->getVotesForUser($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]