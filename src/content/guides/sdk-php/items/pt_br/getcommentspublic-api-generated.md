req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| page | integer | query | Não |  |
| direction | string | query | Não |  |
| sso | string | query | Não |  |
| skip | integer | query | Não |  |
| skipChildren | integer | query | Não |  |
| limit | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| countChildren | boolean | query | Não |  |
| fetchPageForCommentId | string | query | Não |  |
| includeConfig | boolean | query | Não |  |
| countAll | boolean | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| modules | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeNotificationCount | boolean | query | Não |  |
| asTree | boolean | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| useFullTranslationIds | boolean | query | Não |  |
| parentId | string | query | Não |  |
| searchText | string | query | Não |  |
| hashTags | array | query | Não |  |
| userId | string | query | Não |  |
| customConfigStr | string | query | Não |  |
| afterCommentId | string | query | Não |  |
| beforeCommentId | string | query | Não |  |

## Resposta

Retorna: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se você quiser usar um cliente HTTP personalizado, passe seu cliente que implementa `GuzzleHttp\ClientInterface`.
    // Isto é opcional, `GuzzleHttp\Client` será usado como padrão.
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