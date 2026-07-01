αίτηση  
tenantId  
urlId  

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
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

## Απόκριση

Επιστρέφει: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Εάν θέλετε να χρησιμοποιήσετε προσαρμοσμένο http client, περάστε το client σας που υλοποιεί `GuzzleHttp\ClientInterface`.
    // Αυτό είναι προαιρετικό, `GuzzleHttp\Client` θα χρησιμοποιηθεί ως προεπιλογή.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // συμβολοσειρά
$url_id = 'url_id_example'; // συμβολοσειρά
$options = [
    'page' => 56, // ακέραιος
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'sso' => 'sso_example', // συμβολοσειρά
    'skip' => 56, // ακέραιος
    'skip_children' => 56, // ακέραιος
    'limit' => 56, // ακέραιος
    'limit_children' => 56, // ακέραιος
    'count_children' => True, // boolean
    'fetch_page_for_comment_id' => 'fetch_page_for_comment_id_example', // συμβολοσειρά
    'include_config' => True, // boolean
    'count_all' => True, // boolean
    'includei10n' => True, // boolean
    'locale' => 'locale_example', // συμβολοσειρά
    'modules' => 'modules_example', // συμβολοσειρά
    'is_crawler' => True, // boolean
    'include_notification_count' => True, // boolean
    'as_tree' => True, // boolean
    'max_tree_depth' => 56, // ακέραιος
    'use_full_translation_ids' => True, // boolean
    'parent_id' => 'parent_id_example', // συμβολοσειρά
    'search_text' => 'search_text_example', // συμβολοσειρά
    'hash_tags' => array('hash_tags_example'), // string[]
    'user_id' => 'user_id_example', // συμβολοσειρά
    'custom_config_str' => 'custom_config_str_example', // συμβολοσειρά
    'after_comment_id' => 'after_comment_id_example', // συμβολοσειρά
    'before_comment_id' => 'before_comment_id_example', // συμβολοσειρά
];


try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]