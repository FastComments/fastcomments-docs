req
tenantId
urlId

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|------|-------------|
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

## 응답

Returns: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## 예시

[inline-code-attrs-start title = 'getCommentsPublic 예시'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하십시오.
    // 이는 선택 사항이며 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$options = [
    'page' => 56, // int
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // string
    'skip' => 56, // int
    'skip_children' => 56, // int
    'limit' => 56, // int
    'limit_children' => 56, // int
    'count_children' => True, // bool
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // string
    'include_config' => True, // bool
    'count_all' => True, // bool
    'includei10n' => True, // bool
    'locale' => 'locale_example', // string
    'modules' => 'modules_example', // string
    'is_crawler' => True, // bool
    'include_notification_count' => True, // bool
    'as_tree' => True, // bool
    'max_tree_depth' => 56, // int
    'use_full_translation_ids' => True, // bool
    'parent_id' => 'parent_id_example', // string
    'search_text' => 'search_text_example', // string
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // string
    'custom_config_str' => 'custom_config_str_example', // string
    'after_comment_id' => 'after_comment_id_example', // string
    'before_comment_id' => 'before_comment_id_example', // string
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]