Past commenters on the page who are NOT currently online. Sorted by displayName.  
Use this after exhausting /users/online to render a "Members" section.  
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}  
index from afterName forward via $gt, no $skip cost.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή). |
| afterName | string | query | Όχι | Δρομέας: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | Όχι | Δρομέας διαλύτης ισοτιμίας: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName είναι ορισμένο ώστε τα ισοδύναμα ονόματα να μην παραλείπουν εγγραφές. |

## Απόκριση

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, θα χρησιμοποιηθεί το `GuzzleHttp\Client` ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Αναγνωριστικό URL σελίδας (καθαρισμένο από τον διακομιστή).
$options = [
    'after_name' => 'after_name_example', // string | Δρομέας: περάστε το nextAfterName από την προηγούμενη απόκριση.
    'after_user_id' => 'after_user_id_example', // string | Δρομέας διαλύτης ισοτιμίας: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName είναι ορισμένο ώστε τα ισοδύναμα ονόματα να μην παραλείπουν εγγραφές.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]