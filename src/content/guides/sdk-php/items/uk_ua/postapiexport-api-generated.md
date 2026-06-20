## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Ні |  |
| byIPFromComment | string | query | Ні |  |
| filters | string | query | Ні |  |
| searchFilters | string | query | Ні |  |
| sorts | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад postApiExport'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // рядок
$by_ip_from_comment = 'by_ip_from_comment_example'; // рядок
$filters = 'filters_example'; // рядок
$search_filters = 'search_filters_example'; // рядок
$sorts = 'sorts_example'; // рядок
$sso = 'sso_example'; // рядок

try {
    $result = $apiInstance->postApiExport($text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postApiExport: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]