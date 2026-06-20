## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filter | string | query | Не |  |
| searchFilters | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICountCommentsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевано ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filter = 'filter_example'; // string
$search_filters = 'search_filters_example'; // string
$demo = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCount($text_search, $by_ip_from_comment, $filter, $search_filters, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]