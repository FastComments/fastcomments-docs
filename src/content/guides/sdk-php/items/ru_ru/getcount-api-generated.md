## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filter | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| demo | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICountCommentsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример использования getCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // строка
$by_ip_from_comment = 'by_ip_from_comment_example'; // строка
$filter = 'filter_example'; // строка
$search_filters = 'search_filters_example'; // строка
$demo = True; // булево
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->getCount($text_search, $by_ip_from_comment, $filter, $search_filters, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]