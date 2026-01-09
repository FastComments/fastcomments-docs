## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadge200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με API key: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε το παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, εάν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί `GuzzleHttp\Client` ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getUserBadge($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---