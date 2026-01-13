## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Renvoie : [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionConfig200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getQuestionConfig($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]