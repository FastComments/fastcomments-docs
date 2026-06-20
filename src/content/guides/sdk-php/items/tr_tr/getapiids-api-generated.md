## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| text-search | string | query | Hayır |  |
| byIPFromComment | string | query | Hayır |  |
| filters | string | query | Hayır |  |
| searchFilters | string | query | Hayır |  |
| afterId | string | query | Hayır |  |
| demo | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentIdsResponse.php)

## Örnek

[inline-code-attrs-start title = 'getApiIds Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` arayüzünü uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$after_id = 'after_id_example'; // string
$demo = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getApiIds($text_search, $by_ip_from_comment, $filters, $search_filters, $after_id, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiIds: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]