## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|--------------|---------------|------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## Приклад

[inline-code-attrs-start title = 'postApiExport Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати кастомний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$options = [
    'text_search' => 'text_search_example', // рядок
    'by_ip_from_comment' => 'by_ip_from_comment_example', // рядок
    'filters' => 'filters_example', // рядок
    'search_filters' => 'search_filters_example', // рядок
    'sorts' => 'sorts_example', // рядок
    'sso' => 'sso_example', // рядок
];


try {
    $result = $apiInstance->postApiExport($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postApiExport: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]