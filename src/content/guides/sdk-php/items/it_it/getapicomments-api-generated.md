## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getApiComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` sarà usato come predefinito.
    new GuzzleHttp\Client()
);
$page = 3.4; // float
$count = 3.4; // float
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$sorts = 'sorts_example'; // string
$demo = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]