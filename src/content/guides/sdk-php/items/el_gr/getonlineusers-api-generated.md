Τώρα-ενεργοί θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή τη στιγμή.
Επιστρέφει anonCount + totalCount (συνδρομητές σε ολόκληρο το δωμάτιο, συμπεριλαμβανομένων ανώνυμων θεατών που δεν καταμετρούνται ατομικά).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Page URL identifier (cleaned server-side). |
| afterName | string | query | Όχι | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | Όχι | Tiebreaker δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονόματος να μην προκαλούν απώλεια εγγραφών. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί την `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, ο `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Page URL identifier (cleaned server-side).
$after_name = 'after_name_example'; // string | Δείκτης: περάστε το nextAfterName από την προηγούμενη απάντηση.
$after_user_id = 'after_user_id_example'; // string | Tiebreaker δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν το afterName είναι ορισμένο ώστε οι ισοβαθμίες ονόματος να μην διαγράφουν εγγραφές.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]