## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| commentId | string | path | Ναι |  |
| broadcastId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`LockComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/LockComment200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα lockComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί από προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // συμβολοσειρά
$comment_id = 'comment_id_example'; // συμβολοσειρά
$broadcast_id = 'broadcast_id_example'; // συμβολοσειρά
$sso = 'sso_example'; // συμβολοσειρά

try {
    $result = $apiInstance->lockComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->lockComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---