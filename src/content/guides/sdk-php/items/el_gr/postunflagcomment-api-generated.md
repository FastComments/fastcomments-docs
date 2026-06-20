## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postUnFlagComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postUnFlagComment($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postUnFlagComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---