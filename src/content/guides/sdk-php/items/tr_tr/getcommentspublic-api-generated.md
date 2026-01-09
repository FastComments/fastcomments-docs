req
tenantId
urlId

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| direction | string | query | Hayır |  |
| sso | string | query | Hayır |  |
| skip | integer | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| countChildren | boolean | query | Hayır |  |
| fetchPageForCommentId | string | query | Hayır |  |
| includeConfig | boolean | query | Hayır |  |
| countAll | boolean | query | Hayır |  |
| includei10n | boolean | query | Hayır |  |
| locale | string | query | Hayır |  |
| modules | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeNotificationCount | boolean | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| useFullTranslationIds | boolean | query | Hayır |  |
| parentId | string | query | Hayır |  |
| searchText | string | query | Hayır |  |
| hashTags | array | query | Hayır |  |
| userId | string | query | Hayır |  |
| customConfigStr | string | query | Hayır |  |
| afterCommentId | string | query | Hayır |  |
| beforeCommentId | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsPublic200Response.php)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
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