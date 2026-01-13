## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Όχι |  |
| userId | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| questionId | string | query | Όχι |  |
| questionIds | string | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResults200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Καταργήστε το σχόλιο παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί ως προεπιλογή το `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$user_id = 'user_id_example'; // string
$start_date = 'start_date_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = 'question_ids_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getQuestionResults($tenant_id, $url_id, $user_id, $start_date, $question_id, $question_ids, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]