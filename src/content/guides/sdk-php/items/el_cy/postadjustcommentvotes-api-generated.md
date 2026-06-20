## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AdjustVotesResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postAdjustCommentVotes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // συμβολοσειρά
$adjust_comment_votes_params = new \FastComments\Client\Model\AdjustCommentVotesParams(); // \FastComments\Client\Model\AdjustCommentVotesParams
$sso = 'sso_example'; // συμβολοσειρά

try {
    $result = $apiInstance->postAdjustCommentVotes($comment_id, $adjust_comment_votes_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postAdjustCommentVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]