req
tenantId
urlId

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니오 |  |
| direction | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| skip | integer | query | 아니오 |  |
| skipChildren | integer | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| limitChildren | integer | query | 아니오 |  |
| countChildren | boolean | query | 아니오 |  |
| fetchPageForCommentId | string | query | 아니오 |  |
| includeConfig | boolean | query | 아니오 |  |
| countAll | boolean | query | 아니오 |  |
| includei10n | boolean | query | 아니오 |  |
| locale | string | query | 아니오 |  |
| modules | string | query | 아니오 |  |
| isCrawler | boolean | query | 아니오 |  |
| includeNotificationCount | boolean | query | 아니오 |  |
| asTree | boolean | query | 아니오 |  |
| maxTreeDepth | integer | query | 아니오 |  |
| useFullTranslationIds | boolean | query | 아니오 |  |
| parentId | string | query | 아니오 |  |
| searchText | string | query | 아니오 |  |
| hashTags | array | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| customConfigStr | string | query | 아니오 |  |
| afterCommentId | string | query | 아니오 |  |
| beforeCommentId | string | query | 아니오 |  |

## 응답

반환: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
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