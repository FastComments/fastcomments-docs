## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tag | string | path | Sì |  |
| tenantId | string | query | No |  |

## Risposta

Restituisce: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateHashTagResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di patchHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autenticazione con chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Decommentare la riga sottostante per impostare un prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se desideri usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // string
$tenant_id = 'tenant_id_example'; // string
$update_hash_tag_body = new \FastComments\Client\Model\UpdateHashTagBody(); // \FastComments\Client\Model\UpdateHashTagBody

try {
    $result = $apiInstance->patchHashTag($tag, $tenant_id, $update_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---