req
tenantId
urlId

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| page | integer | query | Нет |  |
| direction | string | query | Нет |  |
| sso | string | query | Нет |  |
| skip | integer | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| countChildren | boolean | query | Нет |  |
| fetchPageForCommentId | string | query | Нет |  |
| includeConfig | boolean | query | Нет |  |
| countAll | boolean | query | Нет |  |
| includei10n | boolean | query | Нет |  |
| locale | string | query | Нет |  |
| modules | string | query | Нет |  |
| isCrawler | boolean | query | Нет |  |
| includeNotificationCount | boolean | query | Нет |  |
| asTree | boolean | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| useFullTranslationIds | boolean | query | Нет |  |
| parentId | string | query | Нет |  |
| searchText | string | query | Нет |  |
| hashTags | array | query | Нет |  |
| userId | string | query | Нет |  |
| customConfigStr | string | query | Нет |  |
| afterCommentId | string | query | Нет |  |
| beforeCommentId | string | query | Нет |  |

## Ответ

Возвращает: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsPublic200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
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