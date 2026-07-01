---
req
tenantId
urlId

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| page | integer | query | No |  |
| direction | string | query | No |  |
| sso | string | query | No |  |
| skip | integer | query | No |  |
| skipChildren | integer | query | No |  |
| limit | integer | query | No |  |
| limitChildren | integer | query | No |  |
| countChildren | boolean | query | No |  |
| fetchPageForCommentId | string | query | No |  |
| includeConfig | boolean | query | No |  |
| countAll | boolean | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| modules | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeNotificationCount | boolean | query | No |  |
| asTree | boolean | query | No |  |
| maxTreeDepth | integer | query | No |  |
| useFullTranslationIds | boolean | query | No |  |
| parentId | string | query | No |  |
| searchText | string | query | No |  |
| hashTags | array | query | No |  |
| userId | string | query | No |  |
| customConfigStr | string | query | No |  |
| afterCommentId | string | query | No |  |
| beforeCommentId | string | query | No |  |

## Response

返回：[`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Example

[inline-code-attrs-start title = 'getCommentsPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 字串
$url_id = 'url_id_example'; // 字串
$options = [
    'page' => 56, // 整數
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // 字串
    'skip' => 56, // 整數
    'skip_children' => 56, // 整數
    'limit' => 56, // 整數
    'limit_children' => 56, // 整數
    'count_children' => True, // 布林
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // 字串
    'include_config' => True, // 布林
    'count_all' => True, // 布林
    'includei10n' => True, // 布林
    'locale' => 'locale_example', // 字串
    'modules' => 'modules_example', // 字串
    'is_crawler' => True, // 布林
    'include_notification_count' => True, // 布林
    'as_tree' => True, // 布林
    'max_tree_depth' => 56, // 整數
    'use_full_translation_ids' => True, // 布林
    'parent_id' => 'parent_id_example', // 字串
    'search_text' => 'search_text_example', // 字串
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // 字串
    'custom_config_str' => 'custom_config_str_example', // 字串
    'after_comment_id' => 'after_comment_id_example', // 字串
    'before_comment_id' => 'before_comment_id_example', // 字串
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---