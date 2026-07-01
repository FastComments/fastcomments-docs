## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| urlId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetVotesForUserResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getVotesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Διαμορφώστε την εξουσιοδότηση κλειδιού API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Αφαίρεστε το σχόλιο παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν απαιτείται


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$options = [
    'user_id' => 'user_id_example', // string
    'anon_user_id' => 'anon_user_id_example', // string
];


try {
    $result = $apiInstance->getVotesForUser($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getVotesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]