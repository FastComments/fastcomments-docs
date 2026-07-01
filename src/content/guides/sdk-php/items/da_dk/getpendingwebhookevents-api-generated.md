## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nej |  |
| externalId | string | query | Nej |  |
| eventType | string | query | Nej |  |
| type | string | query | Nej |  |
| domain | string | query | Nej |  |
| attemptCountGT | number | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returns: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventsResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEvents Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisering: api_key
// Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) til API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // streng
$options = [
    'comment_id' => 'comment_id_example', // streng
    'external_id' => 'external_id_example', // streng
    'event_type' => 'event_type_example', // streng
    'type' => 'type_example', // streng
    'domain' => 'domain_example', // streng
    'attempt_count_gt' => 3.4, // flydende
    'skip' => 3.4, // flydende
];


try {
    $result = $apiInstance->getPendingWebhookEvents($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEvents: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]