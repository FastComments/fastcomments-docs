## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| urlId | string | query | Non |  |
| fromCommentId | string | query | Non |  |
| viewed | boolean | query | Non |  |
| type | string | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotifications200Response.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getNotifications'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$url_id = 'url_id_example'; // string
$from_comment_id = 'from_comment_id_example'; // string
$viewed = True; // bool
$type = 'type_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getNotifications($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]