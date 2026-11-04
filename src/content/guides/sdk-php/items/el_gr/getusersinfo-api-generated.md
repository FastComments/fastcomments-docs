Μαζικές πληροφορίες χρηστών για έναν tenant. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από το User / SSOUser.
Χρησιμοποιείται από το widget σχολίων για τον εμπλουτισμό χρηστών που μόλις εμφανίστηκαν μέσω ενός presence event.
Χωρίς πλαίσιο σελίδας: η ιδιωτικότητα επιβάλλεται ομοιόμορφα (τα ιδιωτικά προφίλ καλύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| ids | string | query | Ναι | userIds διαχωρισμένα με κόμμα. |

## Απάντηση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον δικό σας client που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds διαχωρισμένα με κόμμα.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]