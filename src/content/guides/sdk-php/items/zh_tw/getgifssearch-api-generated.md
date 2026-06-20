## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| search | string | query | 是 |  |
| locale | string | query | 否 |  |
| rating | string | query | 否 |  |
| page | number | query | 否 |  |

## 回應

回傳: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsSearchResponse.php)

## 範例

[inline-code-attrs-start title = 'getGifsSearch 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 http 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，`GuzzleHttp\Client` 將會作為預設使用。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字串
$search = 'search_example'; // 字串
$locale = 'locale_example'; // 字串
$rating = 'rating_example'; // 字串
$page = 3.4; // 浮點數

try {
    $result = $apiInstance->getGifsSearch($tenant_id, $search, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsSearch: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]