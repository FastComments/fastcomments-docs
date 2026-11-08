req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| direction | string | query | नहीं |  |
| sso | string | query | नहीं |  |
| skip | integer | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| countChildren | boolean | query | नहीं |  |
| fetchPageForCommentId | string | query | नहीं |  |
| includeConfig | boolean | query | नहीं |  |
| countAll | boolean | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| modules | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeNotificationCount | boolean | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |
| parentId | string | query | नहीं |  |
| searchText | string | query | नहीं |  |
| hashTags | array | query | नहीं |  |
| userId | string | query | नहीं |  |
| customConfigStr | string | query | नहीं |  |
| afterCommentId | string | query | नहीं |  |
| beforeCommentId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentsResponseWithPresencePublicComment.php)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsPublic उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, डिफ़ॉल्ट रूप से `GuzzleHttp\Client` का उपयोग किया जाएगा।
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