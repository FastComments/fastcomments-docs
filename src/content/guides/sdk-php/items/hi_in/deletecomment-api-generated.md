## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| contextUserId | string | query | नहीं |  |
| isLive | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentResult.php)

## उदाहरण

[inline-code-attrs-start title = 'deleteComment उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API की के लिए प्रीफ़िक्स (जैसे Bearer) सेटअप करने के लिए नीचे को अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http क्लाइंट का उपयोग करना चाहते हैं, तो अपना क्लाइंट पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता है।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$options = [
    'context_user_id' => 'context_user_id_example', // string
    'is_live' => True, // bool
];


try {
    $result = $apiInstance->deleteComment($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]