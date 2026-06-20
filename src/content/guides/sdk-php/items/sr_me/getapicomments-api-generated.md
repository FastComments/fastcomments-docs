## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Не |  |
| count | number | query | Не |  |
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Пример

[inline-code-attrs-start title = 'getApiComments Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);
$page = 3.4; // број (float)
$count = 3.4; // број (float)
$text_search = 'text_search_example'; // низ (string)
$by_ip_from_comment = 'by_ip_from_comment_example'; // низ (string)
$filters = 'filters_example'; // низ (string)
$search_filters = 'search_filters_example'; // низ (string)
$sorts = 'sorts_example'; // низ (string)
$demo = True; // бул (bool)
$sso = 'sso_example'; // низ (string)

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]