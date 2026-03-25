## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| state | number | query | Όχι |  |
| skip | number | query | Όχι |  |
| limit | number | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTickets200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTickets'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Αυτό είναι προαιρετικό, η `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$state = 3.4; // float
$skip = 3.4; // float
$limit = 3.4; // float

try {
    $result = $apiInstance->getTickets($tenant_id, $user_id, $state, $skip, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]