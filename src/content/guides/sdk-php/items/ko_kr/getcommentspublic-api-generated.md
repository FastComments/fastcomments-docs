req
tenantId
urlId

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니요 |  |
| direction | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |
| skip | integer | query | 아니요 |  |
| skipChildren | integer | query | 아니요 |  |
| limit | integer | query | 아니요 |  |
| limitChildren | integer | query | 아니요 |  |
| countChildren | boolean | query | 아니요 |  |
| fetchPageForCommentId | string | query | 아니요 |  |
| includeConfig | boolean | query | 아니요 |  |
| countAll | boolean | query | 아니요 |  |
| includei10n | boolean | query | 아니요 |  |
| locale | string | query | 아니요 |  |
| modules | string | query | 아니요 |  |
| isCrawler | boolean | query | 아니요 |  |
| includeNotificationCount | boolean | query | 아니요 |  |
| asTree | boolean | query | 아니요 |  |
| maxTreeDepth | integer | query | 아니요 |  |
| useFullTranslationIds | boolean | query | 아니요 |  |
| parentId | string | query | 아니요 |  |
| searchText | string | query | 아니요 |  |
| hashTags | array | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| customConfigStr | string | query | 아니요 |  |
| afterCommentId | string | query | 아니요 |  |
| beforeCommentId | string | query | 아니요 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsPublic200Response.php)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택사항입니다. 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 문자열
$url_id = 'url_id_example'; // 문자열
$page = 56; // 정수
$direction = new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(); // \FastComments\Client\Model\SortDirections
$sso = 'sso_example'; // 문자열
$skip = 56; // 정수
$skip_children = 56; // 정수
$limit = 56; // 정수
$limit_children = 56; // 정수
$count_children = True; // 불리언
$fetch_page_for_comment_id = 'fetch_page_for_comment_id_example'; // 문자열
$include_config = True; // 불리언
$count_all = True; // 불리언
$includei10n = True; // 불리언
$locale = 'locale_example'; // 문자열
$modules = 'modules_example'; // 문자열
$is_crawler = True; // 불리언
$include_notification_count = True; // 불리언
$as_tree = True; // 불리언
$max_tree_depth = 56; // 정수
$use_full_translation_ids = True; // 불리언
$parent_id = 'parent_id_example'; // 문자열
$search_text = 'search_text_example'; // 문자열
$hash_tags = array('hash_tags_example'); // 문자열[]
$user_id = 'user_id_example'; // 문자열
$custom_config_str = 'custom_config_str_example'; // 문자열
$after_comment_id = 'after_comment_id_example'; // 문자열
$before_comment_id = 'before_comment_id_example'; // 문자열

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $page, $direction, $sso, $skip, $skip_children, $limit, $limit_children, $count_children, $fetch_page_for_comment_id, $include_config, $count_all, $includei10n, $locale, $modules, $is_crawler, $include_notification_count, $as_tree, $max_tree_depth, $use_full_translation_ids, $parent_id, $search_text, $hash_tags, $user_id, $custom_config_str, $after_comment_id, $before_comment_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]