Συνεχώς σε απευθείας σύνδεση θεατές μιας σελίδας: άτομα των οποίων η συνεδρία websocket είναι εγγεγραμμένη στη σελίδα αυτή τη στιγμή.  
Επιστρέφει anonCount + totalCount (συνδρομητές σε ολόκληρο το δωμάτιο, συμπεριλαμβανομένων των ανώνυχων θεατών που δεν απαριθμούμε).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρισμένο στη διακομιστή). |
| afterName | string | query | No | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Διαχωριστικό κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί afterName ώστε οι ισοδύναμες ονομασίες να μην αφαιρούν εγγραφές. |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Αναγνωριστικό URL σελίδας (καθαρισμένο στη διακομιστή).
$options = [
    'after_name' => 'after_name_example', // string | Κέρσορας: περάστε το nextAfterName από την προηγούμενη απάντηση.
    'after_user_id' => 'after_user_id_example', // string | Διαχωριστικό κέρσορα: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί afterName ώστε οι ισοδύναμες ονομασίες να μην αφαιρούν εγγραφές.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]