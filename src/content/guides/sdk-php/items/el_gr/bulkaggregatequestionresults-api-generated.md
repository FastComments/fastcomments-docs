## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| forceRecalculate | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkAggregateQuestionResults200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα bulkAggregateQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$bulk_aggregate_question_results_request = new \FastComments\Client\Model\BulkAggregateQuestionResultsRequest(); // \FastComments\Client\Model\BulkAggregateQuestionResultsRequest
$force_recalculate = True; // bool

try {
    $result = $apiInstance->bulkAggregateQuestionResults($tenant_id, $bulk_aggregate_question_results_request, $force_recalculate);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->bulkAggregateQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]