## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tag | string | path | Ja |  |
| tenantId | string | query | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Eksempel

[inline-code-attrs-start title = 'deleteHashTag Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentar nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du sende din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // string
$tenant_id = 'tenant_id_example'; // string
$delete_hash_tag_request_body = new \FastComments\Client\Model\DeleteHashTagRequestBody(); // \FastComments\Client\Model\DeleteHashTagRequestBody

try {
    $result = $apiInstance->deleteHashTag($tag, $tenant_id, $delete_hash_tag_request_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]