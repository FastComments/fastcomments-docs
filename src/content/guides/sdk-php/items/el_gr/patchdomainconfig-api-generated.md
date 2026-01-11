## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| domainToUpdate | string | path | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfig200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί εξ ορισμού ο `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain_to_update = 'domain_to_update_example'; // string
$patch_domain_config_params = new \FastComments\Client\Model\PatchDomainConfigParams(); // \FastComments\Client\Model\PatchDomainConfigParams

try {
    $result = $apiInstance->patchDomainConfig($tenant_id, $domain_to_update, $patch_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]