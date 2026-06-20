## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| page | number | query | No |  |

## Risposta

Restituisce: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetHashTagsResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getHashTags'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configura l'autorizzazione tramite chiave API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Se vuoi usare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato come predefinito `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getHashTags($tenant_id, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getHashTags: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]