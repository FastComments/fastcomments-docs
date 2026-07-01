## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Response

Retourneert: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventsResponse.php)

## Example

[inline-code-attrs-start title = 'getPendingWebhookEvents Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutel autorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Verwijder commentaar hieronder om prefix in te stellen (bijv. Bearer) voor API-sleutel, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als je een aangepaste http-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt als standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'comment_id' => 'comment_id_example', // string
    'external_id' => 'external_id_example', // string
    'event_type' => 'event_type_example', // string
    'type' => 'type_example', // string
    'domain' => 'domain_example', // string
    'attempt_count_gt' => 3.4, // float
    'skip' => 3.4, // float
];


try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Uitzondering bij het aanroepen van DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]