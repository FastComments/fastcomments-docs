req
tenantId
urlId

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | ruta | Sí |  |
| urlId | string | consulta | Sí |  |
| page | integer | consulta | No |  |
| direction | string | consulta | No |  |
| sso | string | consulta | No |  |
| skip | integer | consulta | No |  |
| skipChildren | integer | consulta | No |  |
| limit | integer | consulta | No |  |
| limitChildren | integer | consulta | No |  |
| countChildren | boolean | consulta | No |  |
| fetchPageForCommentId | string | consulta | No |  |
| includeConfig | boolean | consulta | No |  |
| countAll | boolean | consulta | No |  |
| includei10n | boolean | consulta | No |  |
| locale | string | consulta | No |  |
| modules | string | consulta | No |  |
| isCrawler | boolean | consulta | No |  |
| includeNotificationCount | boolean | consulta | No |  |
| asTree | boolean | consulta | No |  |
| maxTreeDepth | integer | consulta | No |  |
| useFullTranslationIds | boolean | consulta | No |  |
| parentId | string | consulta | No |  |
| searchText | string | consulta | No |  |
| hashTags | array | consulta | No |  |
| userId | string | consulta | No |  |
| customConfigStr | string | consulta | No |  |
| afterCommentId | string | consulta | No |  |
| beforeCommentId | string | consulta | No |  |

## Respuesta

Devuelve: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si desea usar un cliente HTTP personalizado, pase su cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional; se usará `GuzzleHttp\Client` por defecto.
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
    $result = the apiInstance->getCommentsPublic($tenant_id, $url_id, $page, $direction, $sso, $skip, $skip_children, $limit, $limit_children, $count_children, $fetch_page_for_comment_id, $include_config, $count_all, $includei10n, $locale, $modules, $is_crawler, $include_notification_count, $as_tree, $max_tree_depth, $use_full_translation_ids, $parent_id, $search_text, $hash_tags, $user_id, $custom_config_str, $after_comment_id, $before_comment_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]