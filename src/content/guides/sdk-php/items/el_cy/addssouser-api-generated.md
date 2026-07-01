## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-------------|
| tenantId | string | query | Yes |  |

## Απόκριση

Επιστρέφει: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddSSOUserAPIResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addSSOUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Απενεργοποιήστε το σχόλιο παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν απαιτείται


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // συμβολοσειρά
$create_apisso_user_data = new \FastComments\Client\Model\CreateAPISSOUserData(); // \FastComments\Client\Model\CreateAPISSOUserData


try {
    $result = $apiInstance->addSSOUser($tenant_id, $create_apisso_user_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]