## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentId | string | query | Não |  |
| externalId | string | query | Não |  |
| eventType | string | query | Não |  |
| type | string | query | Não |  |
| domain | string | query | Não |  |
| attemptCountGT | number | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEvents200Response.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPendingWebhookEvents'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$external_id = 'external_id_example'; // string
$event_type = 'event_type_example'; // string
$type = 'type_example'; // string
$domain = 'domain_example'; // string
$attempt_count_gt = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $comment_id, $external_id, $event_type, $type, $domain, $attempt_count_gt, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]