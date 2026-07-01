## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nej |  |
| badgeId | string | query | Nej |  |
| type | number | query | Nej |  |
| displayedOnComments | boolean | query | Nej |  |
| limit | number | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgesResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getUserBadges Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgle autorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du videregive din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // streng
$options = [
    'user_id' => 'user_id_example', // streng
    'badge_id' => 'badge_id_example', // streng
    'type' => 3.4, // flydende
    'displayed_on_comments' => True, // bool
    'limit' => 3.4, // flydende
    'skip' => 3.4, // flydende
];


try {
    $result = $apiInstance->getUserBadges($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]