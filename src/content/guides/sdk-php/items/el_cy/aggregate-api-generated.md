Συγκεντρώνει έγγραφα ομαδοποιώντας τα (αν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διάφορες λειτουργίες (π.χ. sum, countDistinct, avg κ.λπ.).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| parentTenantId | string | query | Όχι |  |
| includeStats | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregationResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AggregationResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης API key: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε τον παρακάτω κώδικα για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό — ως προεπιλογή θα χρησιμοποιηθεί το `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$aggregation_request = new \FastComments\Client\Model\AggregationRequest(); // \FastComments\Client\Model\AggregationRequest
$parent_tenant_id = 'parent_tenant_id_example'; // string
$include_stats = True; // bool

try {
    $result = $apiInstance->aggregate($tenant_id, $aggregation_request, $parent_tenant_id, $include_stats);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->aggregate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]