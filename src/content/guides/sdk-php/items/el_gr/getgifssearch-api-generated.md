## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| search | string | query | Ναι |  |
| locale | string | query | Όχι |  |
| rating | string | query | Όχι |  |
| page | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsSearchResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGifsSearch'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε τον πελάτη σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$search = 'search_example'; // string
$options = [
    'locale' => 'locale_example', // string
    'rating' => 'rating_example', // string
    'page' => 3.4, // float
];


try {
    $result = $apiInstance->getGifsSearch($tenant_id, $search, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsSearch: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]