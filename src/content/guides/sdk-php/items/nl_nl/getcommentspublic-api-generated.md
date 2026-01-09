verplicht
tenantId
urlId

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nee |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |
| skip | integer | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| countChildren | boolean | query | Nee |  |
| fetchPageForCommentId | string | query | Nee |  |
| includeConfig | boolean | query | Nee |  |
| countAll | boolean | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| modules | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeNotificationCount | boolean | query | Nee |  |
| asTree | boolean | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |
| parentId | string | query | Nee |  |
| searchText | string | query | Nee |  |
| hashTags | array | query | Nee |  |
| userId | string | query | Nee |  |
| customConfigStr | string | query | Nee |  |
| afterCommentId | string | query | Nee |  |
| beforeCommentId | string | query | Nee |  |

## Response

Retourneert: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsPublic200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
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