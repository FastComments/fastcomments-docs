## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateUserBadge200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο πελάτη HTTP, περάστε τον πελάτη σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, ο `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_user_badge_params = new \FastComments\Client\Model\CreateUserBadgeParams(); // \FastComments\Client\Model\CreateUserBadgeParams

try {
    $result = $apiInstance->createUserBadge($tenant_id, $create_user_badge_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]