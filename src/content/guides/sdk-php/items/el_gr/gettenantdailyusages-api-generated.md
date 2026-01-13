## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| yearNumber | number | query | Όχι |  |
| monthNumber | number | query | Όχι |  |
| dayNumber | number | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantDailyUsages200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantDailyUsages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$year_number = 3.4; // float
$month_number = 3.4; // float
$day_number = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantDailyUsages($tenant_id, $year_number, $month_number, $day_number, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantDailyUsages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]