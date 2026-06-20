## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSuggestResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchSuggest'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати користувацький HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchSuggest($text_search, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSuggest: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]