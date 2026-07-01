req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
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

## Réponse

Retourne : [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // chaîne
$url_id = 'url_id_example'; // chaîne
$options = [
    'page' => 56, // entier
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // chaîne
    'skip' => 56, // entier
    'skip_children' => 56, // entier
    'limit' => 56, // entier
    'limit_children' => 56, // entier
    'count_children' => True, // booléen
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // chaîne
    'include_config' => True, // booléen
    'count_all' => True, // booléen
    'includei10n' => True, // booléen
    'locale' => 'locale_example', // chaîne
    'modules' => 'modules_example', // chaîne
    'is_crawler' => True, // booléen
    'include_notification_count' => True, // booléen
    'as_tree' => True, // booléen
    'max_tree_depth' => 56, // entier
    'use_full_translation_ids' => True, // booléen
    'parent_id' => 'parent_id_example', // chaîne
    'search_text' => 'search_text_example', // chaîne
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // chaîne
    'custom_config_str' => 'custom_config_str_example', // chaîne
    'after_comment_id' => 'after_comment_id_example', // chaîne
    'before_comment_id' => 'before_comment_id_example', // chaîne
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]