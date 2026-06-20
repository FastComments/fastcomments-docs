## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nee |  |

## Response

Retourneert: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetHashTagsResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getHashTags Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer API-sleutelautorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal onderstaande commentaar weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getHashTags($tenant_id, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getHashTags: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]