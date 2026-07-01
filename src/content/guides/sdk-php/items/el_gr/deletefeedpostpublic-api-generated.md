## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτούμενο | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Απάντηση

Επιστρέφει: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteFeedPostPublicResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteFeedPostPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string (συμβολοσειρά)
$post_id = 'post_id_example'; // string (συμβολοσειρά)
$options = [
    'broadcast_id' => 'broadcast_id_example', // string (συμβολοσειρά)
    'sso' => 'sso_example', // string (συμβολοσειρά)
];


try {
    $result = $apiInstance->deleteFeedPostPublic($tenant_id, $post_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]