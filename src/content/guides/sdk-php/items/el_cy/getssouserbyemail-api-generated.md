## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| email | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUserByEmailAPIResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getSSOUserByEmail'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης με κλειδί API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Αν θέλεις να χρησιμοποιήσεις προσαρμοσμένο HTTP client, πέρασε τον client σου που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$email = 'email_example'; // string

try {
    $result = $apiInstance->getSSOUserByEmail($tenant_id, $email);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUserByEmail: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---