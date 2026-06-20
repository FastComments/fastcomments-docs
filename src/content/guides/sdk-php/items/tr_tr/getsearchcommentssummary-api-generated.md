## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Hayır |  |
| filters | string | query | Hayır |  |
| searchFilters | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## Örnek

[inline-code-attrs-start title = 'getSearchCommentsSummary Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Eğer özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchCommentsSummary($value, $filters, $search_filters, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]