## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| editKey | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentVote200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί τη διεπαφή `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$edit_key = 'edit_key_example'; // string

try {
    $result = $apiInstance->deleteVote($tenant_id, $id, $edit_key);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]