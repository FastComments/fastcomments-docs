## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| page | number | query | Нет |  |
| count | number | query | Нет |  |
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sorts | string | query | Нет |  |
| demo | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, будет использован `GuzzleHttp\Client` по умолчанию.
    new GuzzleHttp\Client()
);
$page = 3.4; // число (float)
$count = 3.4; // число (float)
$text_search = 'text_search_example'; // строка
$by_ip_from_comment = 'by_ip_from_comment_example'; // строка
$filters = 'filters_example'; // строка
$search_filters = 'search_filters_example'; // строка
$sorts = 'sorts_example'; // строка
$demo = True; // логическое (bool)
$sso = 'sso_example'; // строка

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]