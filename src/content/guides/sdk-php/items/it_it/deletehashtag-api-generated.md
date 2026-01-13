## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tag | string | path | Sì |  |
| tenantId | string | query | No |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autorizzazione della chiave API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Rimuovi il commento qui sotto per impostare un prefisso (es. Bearer) per la chiave API, se necessario
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client HTTP personalizzato, passa il client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come impostazione predefinita.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // string
$tenant_id = 'tenant_id_example'; // string
$delete_hash_tag_request = new \FastComments\Client\Model\DeleteHashTagRequest(); // \FastComments\Client\Model\DeleteHashTagRequest

try {
    $result = $apiInstance->deleteHashTag($tag, $tenant_id, $delete_hash_tag_request);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]