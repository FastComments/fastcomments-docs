## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSuggestResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getSearchSuggest'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использован `GuzzleHttp\Client`.
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