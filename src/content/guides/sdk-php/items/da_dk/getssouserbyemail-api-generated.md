## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| email | string | path | Yes |  |

## Svar

Returnerer: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUserByEmailAPIResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getSSOUserByEmail Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgle autorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at opsætte prefix (fx Bearer) for API-nøgle, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en tilpasset http-klient, skal du videregive din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // streng
$email = 'email_example'; // streng


try {
    $result = $apiInstance->getSSOUserByEmail($tenant_id, $email);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUserByEmail: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]