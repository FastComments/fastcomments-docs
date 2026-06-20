---
Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή online. Ταξινομημένοι κατά displayName.
Χρησιμοποιήστε αυτό μετά την εξάντληση του /users/online για να εμφανίσετε μια ενότητα "Μέλη".
Σελιδοποίηση cursor στο commenterName: ο διακομιστής διασχίζει το μερικό ευρετήριο {tenantId, urlId, commenterName} από το afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | No | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες ονομάτων να μην παραλείπονται. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό. Το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή).
$after_name = 'after_name_example'; // string | Cursor: περάστε το nextAfterName από την προηγούμενη απάντηση.
$after_user_id = 'after_user_id_example'; // string | Tiebreaker του cursor: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες ονομάτων να μην παραλείπονται.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---