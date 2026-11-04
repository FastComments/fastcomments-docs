Προηγούμενοι σχολιαστές της σελίδας που ΔΕΝ είναι αυτή τη στιγμή συνδεδεμένοι. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε το αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε μια ενότητα "Members".
Σελιδοποίηση με cursor στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName}
ξεκινώντας από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Cursor ισοβαθμίας: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν ορίζεται το afterName ώστε οι ισοβαθμίες στο όνομα να μην απορρίπτουν εγγραφές. |

## Απάντηση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον δικό σας client που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση.
    'after_user_id' => 'after_user_id_example', // string | Cursor ισοβαθμίας: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν ορίζεται το afterName ώστε οι ισοβαθμίες στο όνομα να μην απορρίπτουν εγγραφές.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]