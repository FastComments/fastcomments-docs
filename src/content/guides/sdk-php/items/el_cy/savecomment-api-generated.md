## Παράμετροι

| Όνομα | Type | Location | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| isLive | boolean | query | Όχι |  |
| doSpamCheck | boolean | query | Όχι |  |
| sendEmails | boolean | query | Όχι |  |
| populateNotifications | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APISaveCommentResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα saveComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
// Αποσχολιάστε την παρακάτω γραμμή για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, αν χρειάζεται
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_comment_params = new \FastComments\Client\Model\CreateCommentParams(); // \FastComments\Client\Model\CreateCommentParams
$is_live = True; // bool
$do_spam_check = True; // bool
$send_emails = True; // bool
$populate_notifications = True; // bool

try {
    $result = $apiInstance->saveComment($tenant_id, $create_comment_params, $is_live, $do_spam_check, $send_emails, $populate_notifications);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]