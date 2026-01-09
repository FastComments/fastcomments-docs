## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BlockFromCommentPublic200Response.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα blockFromCommentPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$public_block_from_comment_params = new \FastComments\Client\Model\PublicBlockFromCommentParams(); // \FastComments\Client\Model\PublicBlockFromCommentParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->blockFromCommentPublic($tenant_id, $comment_id, $public_block_from_comment_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->blockFromCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]