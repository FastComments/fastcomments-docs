## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## Пример

[inline-code-attrs-start title = 'postApiExport Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођен HTTP клијент, предајте ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$sorts = 'sorts_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postApiExport($text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postApiExport: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]