req
tenantId
urlId

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| page | integer | query | 否 |  |
| direction | string | query | 否 |  |
| sso | string | query | 否 |  |
| skip | integer | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| countChildren | boolean | query | 否 |  |
| fetchPageForCommentId | string | query | 否 |  |
| includeConfig | boolean | query | 否 |  |
| countAll | boolean | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| modules | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeNotificationCount | boolean | query | 否 |  |
| asTree | boolean | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |
| parentId | string | query | 否 |  |
| searchText | string | query | 否 |  |
| hashTags | array | query | 否 |  |
| userId | string | query | 否 |  |
| customConfigStr | string | query | 否 |  |
| afterCommentId | string | query | 否 |  |
| beforeCommentId | string | query | 否 |  |

## 响应

返回：[`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsPublic200Response.php)

## 示例

[inline-code-attrs-start title = 'getCommentsPublic 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$page = 56; // int
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$sso = 'sso_example'; // string
$skip = 56; // int
$skip_children = 56; // int
$limit = 56; // int
$limit_children = 56; // int
$count_children = True; // bool
$fetch_page_for_comment_id = 'fetch_page_for_comment_id_example'; // string
$include_config = True; // bool
$count_all = True; // bool
$includei10n = True; // bool
$locale = 'locale_example'; // string
$modules = 'modules_example'; // string
$is_crawler = True; // bool
$include_notification_count = True; // bool
$as_tree = True; // bool
$max_tree_depth = 56; // int
$use_full_translation_ids = True; // bool
$parent_id = 'parent_id_example'; // string
$search_text = 'search_text_example'; // string
$hash_tags = array('hash_tags_example'); // string[]
$user_id = 'user_id_example'; // string
$custom_config_str = 'custom_config_str_example'; // string
$after_comment_id = 'after_comment_id_example'; // string
$before_comment_id = 'before_comment_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $page, $direction, $sso, $skip, $skip_children, $limit, $limit_children, $count_children, $fetch_page_for_comment_id, $include_config, $count_all, $includei10n, $locale, $modules, $is_crawler, $include_notification_count, $as_tree, $max_tree_depth, $use_full_translation_ids, $parent_id, $search_text, $hash_tags, $user_id, $custom_config_str, $after_comment_id, $before_comment_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]