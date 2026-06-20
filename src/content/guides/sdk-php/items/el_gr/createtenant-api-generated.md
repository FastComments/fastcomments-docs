## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTenantResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createTenant'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Καταργήστε το σχόλιο παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί το `GuzzleHttp\Client` ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_tenant_body = new \FastComments\Client\Model\CreateTenantBody(); // \FastComments\Client\Model\CreateTenantBody

try {
    $result = $apiInstance->createTenant($tenant_id, $create_tenant_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTenant: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]