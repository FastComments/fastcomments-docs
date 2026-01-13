## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetUser200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUser200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε πιο κάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε custom http client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, η `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // συμβολοσειρά
$id = 'id_example'; // συμβολοσειρά

try {
    $result = $apiInstance->getUser($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---