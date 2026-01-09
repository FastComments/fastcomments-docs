## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateSubscriptionAPIResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createSubscription'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο πελάτη HTTP, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_api_user_subscription_data = new \FastComments\Client\Model\CreateAPIUserSubscriptionData(); // \FastComments\Client\Model\CreateAPIUserSubscriptionData

try {
    $result = $apiInstance->createSubscription($tenant_id, $create_api_user_subscription_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---