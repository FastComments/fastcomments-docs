## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| deleteComments | string | query | Όχι |  |
| commentDeleteMode | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Αποσχολιάστε την παρακάτω γραμμή για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP πελάτη, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί από προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$delete_comments = 'delete_comments_example'; // string
$comment_delete_mode = 'comment_delete_mode_example'; // string

try {
    $result = $apiInstance->deleteTenantUser($tenant_id, $id, $delete_comments, $comment_delete_mode);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]