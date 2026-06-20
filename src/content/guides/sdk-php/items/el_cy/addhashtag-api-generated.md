## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateHashTagResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε το παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, εάν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, η `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_hash_tag_body = new \FastComments\Client\Model\CreateHashTagBody(); // \FastComments\Client\Model\CreateHashTagBody

try {
    $result = $apiInstance->addHashTag($tenant_id, $create_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]