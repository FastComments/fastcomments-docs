## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getApiComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Αν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, το `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'page' => 3.4, // float
    'count' => 3.4, // float
    'text_search' => 'text_search_example', // string
    'by_ip_from_comment' => 'by_ip_from_comment_example', // string
    'filters' => 'filters_example', // string
    'search_filters' => 'search_filters_example', // string
    'sorts' => 'sorts_example', // string
    'demo' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getApiComments($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]