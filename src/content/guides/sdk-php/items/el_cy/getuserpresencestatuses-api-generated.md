## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlIdWS | string | query | Yes |  |
| userIds | string | query | Yes |  |

## Ανταπόκριση

Επιστρέφει: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatusesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'getUserPresenceStatuses Παράδειγμα'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // συμβολοσειρά
$url_id_ws = 'url_id_ws_example'; // συμβολοσειρά
$user_ids = 'user_ids_example'; // συμβολοσειρά


try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]