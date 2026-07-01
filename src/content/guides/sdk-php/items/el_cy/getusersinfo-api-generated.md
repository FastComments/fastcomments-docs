Πληροφορίες πολλαπλών χρηστών για έναν ενοικιαστή. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από το User / SSOUser. Χρησιμοποιείται από το widget σχολίων για τον εμπλουτισμό των χρηστών που μόλις εμφανίστηκαν μέσω ενός γεγονότος παρουσίας. Χωρίς πλαίσιο σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ καλύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Ταυτογνωριστικά χρηστών διαχωρισμένα με κόμμα. |

## Απόκριση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'getUsersInfo Παράδειγμα'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Ταυτογνωριστικά χρηστών διαχωρισμένα με κόμμα.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]