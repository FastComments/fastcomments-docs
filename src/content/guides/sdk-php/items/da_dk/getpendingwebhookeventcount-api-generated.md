## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|-----------|--------|-------------|
| tenantId | string | forespørgsel | Ja |  |
| commentId | string | forespørgsel | Nej |  |
| externalId | string | forespørgsel | Nej |  |
| eventType | string | forespørgsel | Nej |  |
| type | string | forespørgsel | Nej |  |
| domain | string | forespørgsel | Nej |  |
| attemptCountGT | number | forespørgsel | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventCountResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
// Fjern kommentaren nedenfor for at konfigurere præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en tilpasset HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
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
    'attempt_count_gt' => 3.4, // float
];


try {
    $result = $apiInstance->getPendingWebhookEventCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEventCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]