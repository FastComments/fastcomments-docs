req
tenantId
urlId

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| urlId | string | upit | Da |  |
| page | integer | upit | Ne |  |
| direction | string | upit | Ne |  |
| sso | string | upit | Ne |  |
| skip | integer | upit | Ne |  |
| skipChildren | integer | upit | Ne |  |
| limit | integer | upit | Ne |  |
| limitChildren | integer | upit | Ne |  |
| countChildren | boolean | upit | Ne |  |
| fetchPageForCommentId | string | upit | Ne |  |
| includeConfig | boolean | upit | Ne |  |
| countAll | boolean | upit | Ne |  |
| includei10n | boolean | upit | Ne |  |
| locale | string | upit | Ne |  |
| modules | string | upit | Ne |  |
| isCrawler | boolean | upit | Ne |  |
| includeNotificationCount | boolean | upit | Ne |  |
| asTree | boolean | upit | Ne |  |
| maxTreeDepth | integer | upit | Ne |  |
| useFullTranslationIds | boolean | upit | Ne |  |
| parentId | string | upit | Ne |  |
| searchText | string | upit | Ne |  |
| hashTags | array | upit | Ne |  |
| userId | string | upit | Ne |  |
| customConfigStr | string | upit | Ne |  |
| afterCommentId | string | upit | Ne |  |
| beforeCommentId | string | upit | Ne |  |

## Odgovor

Vraća: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, za zadani će se koristiti `GuzzleHttp\Client`.
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

---