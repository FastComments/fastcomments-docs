## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| skip | number | query | No |  |

## Απόκριση

Επιστρέφει: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplateRenderErrorsResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getEmailTemplateRenderErrors'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Ξεσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί η `GuzzleHttp\Client` ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$skip = 3.4; // float


try {
    $result = $apiInstance->getEmailTemplateRenderErrors($tenant_id, $id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplateRenderErrors: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]