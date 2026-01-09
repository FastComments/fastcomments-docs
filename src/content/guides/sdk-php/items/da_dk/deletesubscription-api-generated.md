## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nej |  |

## Svar

Returnerer: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSubscriptionAPIResponse.php)

## Eksempel

[inline-code-attrs-start title = 'deleteSubscription-eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisering: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentar nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->deleteSubscription($tenant_id, $id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]