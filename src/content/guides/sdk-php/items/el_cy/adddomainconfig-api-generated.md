## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddDomainConfig200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε την παρακάτω γραμμή για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί ως προεπιλογή ο `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$add_domain_config_params = new \FastComments\Client\Model\AddDomainConfigParams(); // \FastComments\Client\Model\AddDomainConfigParams

try {
    $result = $apiInstance->addDomainConfig($tenant_id, $add_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]