## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserBadge200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε την παρακάτω γραμμή για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, ως προεπιλογή θα χρησιμοποιηθεί ο `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->deleteUserBadge($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]