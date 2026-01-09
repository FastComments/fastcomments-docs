req
tenantId
afterId

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| afterId | string | query | Nein |  |
| limit | integer | query | Nein |  |
| tags | array | query | Nein |  |

## Antwort

Gibt zurück: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPosts200Response.php)

## Beispiel

[inline-code-attrs-start title = 'getFeedPosts Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API-Schlüssel-Autorisierung konfigurieren: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Unten auskommentierten Code einkommentieren, um bei Bedarf ein Präfix (z. B. Bearer) für den API-Schlüssel einzustellen
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
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