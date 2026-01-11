## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| updateComments | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`PatchSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PatchSSOUserAPIResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchSSOUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_apisso_user_data = new \FastComments\Client\Model\UpdateAPISSOUserData(); // \FastComments\Client\Model\UpdateAPISSOUserData
$update_comments = True; // bool

try {
    $result = $apiInstance->patchSSOUser($tenant_id, $id, $update_apisso_user_data, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->patchSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]