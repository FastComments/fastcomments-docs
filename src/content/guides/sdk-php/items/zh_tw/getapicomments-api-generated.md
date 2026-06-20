## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
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

## 回應

回傳：[`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## 範例

[inline-code-attrs-start title = 'getApiComments 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$page = 3.4; // 浮點數
$count = 3.4; // 浮點數
$text_search = 'text_search_example'; // 字串
$by_ip_from_comment = 'by_ip_from_comment_example'; // 字串
$filters = 'filters_example'; // 字串
$search_filters = 'search_filters_example'; // 字串
$sorts = 'sorts_example'; // 字串
$demo = True; // 布林值
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---