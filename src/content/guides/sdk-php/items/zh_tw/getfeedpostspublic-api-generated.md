req
tenantId
afterId

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| afterId | string | query | 否 |  |
| limit | integer | query | 否 |  |
| tags | array | query | 否 |  |
| sso | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeUserInfo | boolean | query | 否 |  |

## 回應

回傳: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicFeedPostsResponse.php)

## 範例

[inline-code-attrs-start title = 'getFeedPostsPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，`GuzzleHttp\Client` 將會被用作預設值。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$after_id = 'after_id_example'; // string
$limit = 56; // int
$tags = array('tags_example'); // string[]
$sso = 'sso_example'; // string
$is_crawler = True; // bool
$include_user_info = True; // bool

try {
    $result = $apiInstance->getFeedPostsPublic($tenant_id, $after_id, $limit, $tags, $sso, $is_crawler, $include_user_info);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]