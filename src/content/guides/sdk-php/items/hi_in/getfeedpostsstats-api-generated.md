## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| postIds | array | query | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FeedPostsStatsResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsStats उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता है।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$post_ids = array('post_ids_example'); // string[]
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->getFeedPostsStats($tenant_id, $post_ids, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getFeedPostsStats: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]