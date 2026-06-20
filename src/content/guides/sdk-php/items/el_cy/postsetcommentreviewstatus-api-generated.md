## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| reviewed | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postSetCommentReviewStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, ο `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$reviewed = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentReviewStatus($comment_id, $reviewed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentReviewStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]