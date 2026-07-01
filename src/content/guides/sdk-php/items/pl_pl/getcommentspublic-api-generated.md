req  
tenantId  
urlId  

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| page | integer | query | Nie |  |
| direction | string | query | Nie |  |
| sso | string | query | Nie |  |
| skip | integer | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| countChildren | boolean | query | Nie |  |
| fetchPageForCommentId | string | query | Nie |  |
| includeConfig | boolean | query | Nie |  |
| countAll | boolean | query | Nie |  |
| includei10n | boolean | query | Nie |  |
| locale | string | query | Nie |  |
| modules | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeNotificationCount | boolean | query | Nie |  |
| asTree | boolean | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| useFullTranslationIds | boolean | query | Nie |  |
| parentId | string | query | Nie |  |
| searchText | string | query | Nie |  |
| hashTags | array | query | Nie |  |
| userId | string | query | Nie |  |
| customConfigStr | string | query | Nie |  |
| afterCommentId | string | query | Nie |  |
| beforeCommentId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć własnego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie użyty zostanie `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // ciąg znaków
$url_id = 'url_id_example'; // ciąg znaków
$options = [
    'page' => 56, // liczba całkowita
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // ciąg znaków
    'skip' => 56, // liczba całkowita
    'skip_children' => 56, // liczba całkowita
    'limit' => 56, // liczba całkowita
    'limit_children' => 56, // liczba całkowita
    'count_children' => True, // bool
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // ciąg znaków
    'include_config' => True, // bool
    'count_all' => True, // bool
    'includei10n' => True, // bool
    'locale' => 'locale_example', // ciąg znaków
    'modules' => 'modules_example', // ciąg znaków
    'is_crawler' => True, // bool
    'include_notification_count' => True, // bool
    'as_tree' => True, // bool
    'max_tree_depth' => 56, // liczba całkowita
    'use_full_translation_ids' => True, // bool
    'parent_id' => 'parent_id_example', // ciąg znaków
    'search_text' => 'search_text_example', // ciąg znaków
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // ciąg znaków
    'custom_config_str' => 'custom_config_str_example', // ciąg znaków
    'after_comment_id' => 'after_comment_id_example', // ciąg znaków
    'before_comment_id' => 'before_comment_id_example', // ciąg znaków
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]