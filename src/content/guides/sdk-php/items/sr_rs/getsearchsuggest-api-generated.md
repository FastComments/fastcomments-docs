## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSuggestResponse.php)

## Пример

[inline-code-attrs-start title = 'getSearchSuggest Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако желите да користите прилагођен HTTP клијент, проследите клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевани ће бити `GuzzleHttp\Client`.
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