## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | 否 |  |
| count | number | query | 否 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## 示例

[inline-code-attrs-start title = 'getApiComments 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$page = 3.4; // 浮点数
$count = 3.4; // 浮点数
$text_search = 'text_search_example'; // 字符串
$by_ip_from_comment = 'by_ip_from_comment_example'; // 字符串
$filters = 'filters_example'; // 字符串
$search_filters = 'search_filters_example'; // 字符串
$sorts = 'sorts_example'; // 字符串
$demo = True; // 布尔
$sso = 'sso_example'; // 字符串

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]