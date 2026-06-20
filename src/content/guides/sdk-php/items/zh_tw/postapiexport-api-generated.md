## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## 範例

[inline-code-attrs-start title = 'postApiExport 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // 字串
$by_ip_from_comment = 'by_ip_from_comment_example'; // 字串
$filters = 'filters_example'; // 字串
$search_filters = 'search_filters_example'; // 字串
$sorts = 'sorts_example'; // 字串
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->postApiExport($text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postApiExport: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]