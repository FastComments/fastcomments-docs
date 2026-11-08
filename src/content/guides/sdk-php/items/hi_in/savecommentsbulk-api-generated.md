## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| isLive | boolean | query | नहीं |  |
| doSpamCheck | boolean | query | नहीं |  |
| sendEmails | boolean | query | नहीं |  |
| populateNotifications | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveCommentsBulkResponse.php)

## उदाहरण

[inline-code-attrs-start title = 'saveCommentsBulk उदाहरण'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API key authorization कॉन्फ़िगर करें: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// यदि आवश्यक हो तो API key के लिए prefix (जैसे Bearer) सेटअप करने के लिए नीचे दिया कोड अनकमेंट करें
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // यदि आप कस्टम http client का उपयोग करना चाहते हैं, तो अपना client पास करें जो `GuzzleHttp\ClientInterface` को इम्प्लीमेंट करता हो।
    // यह वैकल्पिक है, `GuzzleHttp\Client` डिफ़ॉल्ट रूप से उपयोग किया जाएगा।
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_comment_params = array(new \FastComments\Client\Model\CreateCommentParams()); // \FastComments\Client\Model\CreateCommentParams[]
$options = [
    'is_live' => True, // bool
    'do_spam_check' => True, // bool
    'send_emails' => True, // bool
    'populate_notifications' => True, // bool
];


try {
    $result = $apiInstance->saveCommentsBulk($tenant_id, $create_comment_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveCommentsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]