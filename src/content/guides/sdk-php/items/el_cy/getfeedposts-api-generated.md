req
tenantId
afterId

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| afterId | string | query | Όχι |  |
| limit | integer | query | Όχι |  |
| tags | array | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getFeedPosts'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, ο `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
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