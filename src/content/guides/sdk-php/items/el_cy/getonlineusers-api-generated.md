Θεατές που είναι αυτή τη στιγμή online σε μια σελίδα: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή τη στιγμή.
Επιστρέφει anonCount + totalCount (συνδρομητές σε όλο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαραίτητο | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | διαδρομή | Ναι |  |
| urlId | string | ερώτημα | Ναι | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | ερώτημα | Όχι | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | ερώτημα | Όχι | Κανόνας επίλυσης ισοβαθμίας δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι καταχωρίσεις με ίδιο όνομα να μην παραλείπονται. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο HTTP client, περάστε τον client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλεγμένο.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Αναγνωριστικό URL της σελίδας (καθαρίζεται από τον διακομιστή).
$after_name = 'after_name_example'; // string | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση.
$after_user_id = 'after_user_id_example'; // string | Κανόνας επίλυσης ισοβαθμίας δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι καταχωρίσεις με ίδιο όνομα να μην παραλείπονται.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]