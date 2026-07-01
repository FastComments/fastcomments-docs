req
tenantId
afterId

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| afterId | string | query | 否 |  |
| limit | integer | query | 否 |  |
| tags | array | query | 否 |  |
| sso | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeUserInfo | boolean | query | 否 |  |

## 回應

返回：[`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicFeedPostsResponse.php)

## 範例

[inline-code-attrs-start title = 'getFeedPostsPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，`GuzzleHttp\Client` 會被預設使用。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 字串
$options = [
    'after_id' => 'after_id_example', // 字串
    'limit' => 56, // 整數
    'tags' => array('tags_example'), // 字串陣列
    'sso' => 'sso_example', // 字串
    'is_crawler' => True, // 布林值
    'include_user_info' => True, // 布林值
];


try {
    $result = $apiInstance->getFeedPostsPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---