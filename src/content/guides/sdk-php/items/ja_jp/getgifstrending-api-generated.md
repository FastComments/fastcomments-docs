## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## レスポンス

戻り値: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## 例

[inline-code-attrs-start title = 'getGifsTrending の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムHTTPクライアントを使いたい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意で、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 文字列
$locale = 'locale_example'; // 文字列
$rating = 'rating_example'; // 文字列
$page = 3.4; // 浮動小数点

try {
    $result = $apiInstance->getGifsTrending($tenant_id, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---