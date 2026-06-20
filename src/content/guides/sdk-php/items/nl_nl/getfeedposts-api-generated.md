req
tenantId
afterId

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nee |  |
| limit | integer | query | Nee |  |
| tags | array | query | Nee |  |

## Respons

Retourneert: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPosts Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configureer de API-sleutelautorisatie: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Haal de commentaar hieronder weg om een prefix (bijv. Bearer) voor de API-sleutel in te stellen, indien nodig
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$after_id = 'after_id_example'; // string
$limit = 56; // int
$tags = array('tags_example'); // string[]

try {
    $result = $apiInstance->getFeedPosts($tenant_id, $after_id, $limit, $tags);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getFeedPosts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]