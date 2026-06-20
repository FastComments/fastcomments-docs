## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| locale | string | 查询 | 否 |  |
| rating | string | 查询 | 否 |  |
| page | number | 查询 | 否 |  |

## 响应

返回: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## 示例

[inline-code-attrs-start title = 'getGifsTrending 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 http 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字符串
$locale = 'locale_example'; // 字符串
$rating = 'rating_example'; // 字符串
$page = 3.4; // 浮点数

try {
    $result = $apiInstance->getGifsTrending($tenant_id, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---