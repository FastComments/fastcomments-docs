Μαζικές πληροφορίες χρηστών για έναν tenant. Δεδομένων των userIds, επιστρέφει πληροφορίες εμφάνισης από User / SSOUser.
Χρησιμοποιείται από το comment widget για να εμπλουτίσει χρήστες που μόλις εμφανίστηκαν μέσω ενός presence event.
Δεν υπάρχει context σελίδας: η ιδιωτικότητα εφαρμόζεται ομοιόμορφα (τα ιδιωτικά προφίλ αποκρύπτονται).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| ids | string | query | Ναι | userIds διαχωρισμένα με κόμμα. |

## Απόκριση

Επιστρέφει: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον client σας που υλοποιεί το `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί ως προεπιλογή το `GuzzleHttp\Client`.
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

---