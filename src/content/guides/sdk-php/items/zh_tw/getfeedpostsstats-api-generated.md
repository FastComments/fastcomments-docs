## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postIds | array | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

Returns: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetFeedPostsStats200Response.php)

## 範例

[inline-code-attrs-start title = 'getFeedPostsStats 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果要使用自訂的 http 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_ids = array('post_ids_example'); // string[]
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getFeedPostsStats($tenant_id, $post_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsStats: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]