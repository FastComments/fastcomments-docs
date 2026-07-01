req
tenantId
urlId

## Parámetros

| Name | Type | Location | Required | Descripción |
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

## Respuesta

Devuelve: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si deseas usar un cliente HTTP personalizado, pasa tu cliente que implemente `GuzzleHttp\ClientInterface`.
    // Esto es opcional, se usará `GuzzleHttp\Client` por defecto.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // cadena
$url_id = 'url_id_example'; // cadena
$options = [
    'page' => 56, // entero
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // cadena
    'skip' => 56, // entero
    'skip_children' => 56, // entero
    'limit' => 56, // entero
    'limit_children' => 56, // entero
    'count_children' => True, // booleano
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // cadena
    'include_config' => True, // booleano
    'count_all' => True, // booleano
    'includei10n' => True, // booleano
    'locale' => 'locale_example', // cadena
    'modules' => 'modules_example', // cadena
    'is_crawler' => True, // booleano
    'include_notification_count' => True, // booleano
    'as_tree' => True, // booleano
    'max_tree_depth' => 56, // entero
    'use_full_translation_ids' => True, // booleano
    'parent_id' => 'parent_id_example', // cadena
    'search_text' => 'search_text_example', // cadena
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // cadena
    'custom_config_str' => 'custom_config_str_example', // cadena
    'after_comment_id' => 'after_comment_id_example', // cadena
    'before_comment_id' => 'before_comment_id_example', // cadena
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]