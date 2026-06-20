---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Ne |  |
| count | number | query | Ne |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sorts | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getApiComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$page = 3.4; // plavajoče število
$count = 3.4; // plavajoče število
$text_search = 'text_search_example'; // niz
$by_ip_from_comment = 'by_ip_from_comment_example'; // niz
$filters = 'filters_example'; // niz
$search_filters = 'search_filters_example'; // niz
$sorts = 'sorts_example'; // niz
$demo = True; // logična vrednost
$sso = 'sso_example'; // niz

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---