## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | query | Sì |  |
| direction | string | query | Sì |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Risposta

Restituisce: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio createVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autenticazione tramite chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Decommenta la riga seguente per impostare un prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se desideri usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come impostazione predefinita.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$direction = 'direction_example'; // string
$user_id = 'user_id_example'; // string
$anon_user_id = 'anon_user_id_example'; // string

try {
    $result = $apiInstance->createVote($tenant_id, $comment_id, $direction, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]