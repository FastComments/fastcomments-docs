## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filter | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| demo | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却値: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICountCommentsResponse.php)

## 例

[inline-code-attrs-start title = 'getCountの例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムの HTTP クライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filter = 'filter_example'; // string
$search_filters = 'search_filters_example'; // string
$demo = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCount($text_search, $by_ip_from_comment, $filter, $search_filters, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]