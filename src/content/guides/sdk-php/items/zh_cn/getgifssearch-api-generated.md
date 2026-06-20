## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| search | string | query | 是 |  |
| locale | string | query | 否 |  |
| rating | string | query | 否 |  |
| page | number | query | 否 |  |

## 响应

返回：[`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsSearchResponse.php)

## 示例

[inline-code-attrs-start title = 'getGifsSearch 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 http 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$search = 'search_example'; // string
$locale = 'locale_example'; // string
$rating = 'rating_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getGifsSearch($tenant_id, $search, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsSearch: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]