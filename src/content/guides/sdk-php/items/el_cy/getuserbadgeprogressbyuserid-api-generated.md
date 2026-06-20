## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgeProgressResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserBadgeProgressByUserId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Ρυθμίστε την εξουσιοδότηση με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Ξε-σχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό — ως προεπιλογή θα χρησιμοποιηθεί το `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->getUserBadgeProgressByUserId($tenant_id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressByUserId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---