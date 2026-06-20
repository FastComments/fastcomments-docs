Παρακάτω σχόλια στη σελίδα από χρήστες που ΔΕΝ είναι αυτή τη στιγμή online. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε μια ενότητα "Members".
Σελίδωση με cursor στο commenterName: ο server διασχίζει το μερικό {tenantId, urlId, commenterName}
index από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Δεύτερο κριτήριο cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοπαλίες στα ονόματα να μην παραλείπουν εγγραφές. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί ως προεπιλογή ο `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
$after_name = 'after_name_example'; // string | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση.
$after_user_id = 'after_user_id_example'; // string | Δεύτερο κριτήριο cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοπαλίες στα ονόματα να μην παραλείπουν εγγραφές.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]