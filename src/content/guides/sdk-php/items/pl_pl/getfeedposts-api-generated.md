req
tenantId
afterId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| afterId | string | query | Nie |  |
| limit | integer | query | Nie |  |
| tags | array | query | Nie |  |

## Odpowiedź

Zwraca: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPosts200Response.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getFeedPosts'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Skonfiguruj autoryzację klucza API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentuj poniżej, aby ustawić prefiks (np. Bearer) dla klucza API, jeśli to konieczne
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż swój klient, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, `GuzzleHttp\Client` will be used as default.
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