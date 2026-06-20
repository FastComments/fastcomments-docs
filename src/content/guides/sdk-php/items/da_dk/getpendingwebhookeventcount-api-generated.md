## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nej |  |
| externalId | string | query | Nej |  |
| eventType | string | query | Nej |  |
| type | string | query | Nej |  |
| domain | string | query | Nej |  |
| attemptCountGT | number | query | Nej |  |

## Svar

Returnerer: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventCountResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at sætte et præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` bruges som standard.
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

try {
    $result = $apiInstance->getPendingWebhookEventCount($tenant_id, $comment_id, $external_id, $event_type, $type, $domain, $attempt_count_gt);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEventCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]