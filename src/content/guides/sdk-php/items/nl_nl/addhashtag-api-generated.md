## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Nee |  |

## Respons

Geeft terug: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddHashTag200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'addHashTag Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer autorisatie voor API-sleutel: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal hieronder de commentaarstreep weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client mee die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_hash_tag_body = new \FastComments\Client\Model\CreateHashTagBody(); // \FastComments\Client\Model\CreateHashTagBody

try {
    $result = $apiInstance->addHashTag($tenant_id, $create_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]