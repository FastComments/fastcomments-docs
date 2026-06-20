## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nej |  |

## Svar

Returnerer: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkCreateHashTagsResponse.php)

## Eksempel

[inline-code-attrs-start title = 'addHashTagsBulk Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøgleautorisation: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at sætte præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du ønsker at bruge en brugerdefineret HTTP-klient, kan du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$bulk_create_hash_tags_body = new \FastComments\Client\Model\BulkCreateHashTagsBody(); // \FastComments\Client\Model\BulkCreateHashTagsBody

try {
    $result = $apiInstance->addHashTagsBulk($tenant_id, $bulk_create_hash_tags_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTagsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]