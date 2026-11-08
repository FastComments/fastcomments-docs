req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| afterId | string | query | नहीं |  |
| limit | integer | query | नहीं |  |
| tags | array | query | नहीं |  |
| sso | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeUserInfo | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicFeedPostsResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsPublic उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, डिफ़ॉल्ट रूप से `GuzzleHttp\Client` का उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'after_id' => 'after_id_example', // string
    'limit' => 56, // int
    'tags' => array('tags_example'), // string[]
    'sso' => 'sso_example', // string
    'is_crawler' => True, // bool
    'include_user_info' => True, // bool
];


try {
    $result = $apiInstance->getFeedPostsPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]