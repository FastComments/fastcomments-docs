## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| skip | integer | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| anonUserId | string | query | नहीं |  |
| contextUserId | string | query | नहीं |  |
| hashTag | string | query | नहीं |  |
| parentId | string | query | नहीं |  |
| direction | string | query | नहीं |  |
| fromDate | integer | query | नहीं |  |
| toDate | integer | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'getComments उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API कुंजी प्राधिकरण कॉन्फ़िगर करें: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API कुंजी के लिए उपसर्ग (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता है।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'page' => 56, // int
    'limit' => 56, // int
    'skip' => 56, // int
    'as_tree' => True, // bool
    'skip_children' => 56, // int
    'limit_children' => 56, // int
    'max_tree_depth' => 56, // int
    'url_id' => 'url_id_example', // string
    'user_id' => 'user_id_example', // string
    'anon_user_id' => 'anon_user_id_example', // string
    'context_user_id' => 'context_user_id_example', // string
    'hash_tag' => 'hash_tag_example', // string
    'parent_id' => 'parent_id_example', // string
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'from_date' => 56, // int
    'to_date' => 56, // int
];


try {
    $result = $apiInstance->getComments($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]